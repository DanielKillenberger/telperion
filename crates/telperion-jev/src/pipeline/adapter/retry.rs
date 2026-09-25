//! Transient fetch failures wait and retry (owner, 2026-09-25).
//!
//! A rate limit is not a missing source: Firecrawl's per-minute limit says
//! how long to wait ("retry after 9s"), and the beech's first run settled
//! two such sources as dropped. `Retrying` wraps any adapter: a rate-limited
//! call waits the delay the error names and tries again, up to `ATTEMPTS`
//! times; once a limit has been reported, calls are paced to stay under it.
//! Only a permanent failure, or a limit that outlasts every attempt, reaches
//! the stage as an error, which files `unavailable-source`.
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::Path;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use regex::Regex;

use super::{AdapterError, FetchAdapter, Scrape, SearchHit, Spent};

/// Tries per call, the first included.
pub const ATTEMPTS: u32 = 4;
/// The wait when a rate limit names none, and the longest one ever taken.
const DEFAULT_WAIT: Duration = Duration::from_secs(10);
const LONGEST_WAIT: Duration = Duration::from_secs(60);
const WINDOW: Duration = Duration::from_secs(60);

pub struct Retrying {
    inner: Box<dyn FetchAdapter>,
    sleep: fn(Duration),
    /// Calls per minute the provider reported as its limit, once it has.
    limit: Cell<Option<usize>>,
    recent: RefCell<VecDeque<Instant>>,
}

impl Retrying {
    pub fn new(inner: Box<dyn FetchAdapter>) -> Self {
        Self::with_sleep(inner, std::thread::sleep)
    }

    /// A test passes a sleep that records instead of waiting.
    pub fn with_sleep(inner: Box<dyn FetchAdapter>, sleep: fn(Duration)) -> Self {
        Self {
            inner,
            sleep,
            limit: Cell::new(None),
            recent: RefCell::new(VecDeque::new()),
        }
    }

    fn call<T>(&self, f: impl Fn() -> Result<T, AdapterError>) -> Result<T, AdapterError> {
        let mut attempt = 1;
        loop {
            self.pace();
            let result = f();
            let limited = result.as_ref().err().and_then(rate_limited);
            match limited {
                Some((wait, limit)) if attempt < ATTEMPTS => {
                    if limit.is_some() {
                        self.limit.set(limit);
                    }
                    (self.sleep)(wait);
                    attempt += 1;
                }
                _ => return result,
            }
        }
    }

    /// Waits, once a limit is known, until one more call stays under it.
    fn pace(&self) {
        let now = Instant::now();
        let mut recent = self.recent.borrow_mut();
        while recent
            .front()
            .is_some_and(|t| now.duration_since(*t) >= WINDOW)
        {
            recent.pop_front();
        }
        if let Some(limit) = self.limit.get() {
            if recent.len() + 1 >= limit {
                if let Some(oldest) = recent.front() {
                    (self.sleep)(WINDOW.saturating_sub(now.duration_since(*oldest)));
                }
            }
        }
        recent.push_back(Instant::now());
    }
}

/// The wait a rate-limit error names, and the per-minute limit it reports,
/// or None when the error is not a rate limit.
pub fn rate_limited(err: &AdapterError) -> Option<(Duration, Option<usize>)> {
    let AdapterError::Failed { error, .. } = err else {
        return None;
    };
    let lower = error.to_ascii_lowercase();
    let limited = ["rate limit", "too many requests", " 429"]
        .iter()
        .any(|m| lower.contains(m));
    if !limited {
        return None;
    }
    static AFTER: OnceLock<Regex> = OnceLock::new();
    static CONSUMED: OnceLock<Regex> = OnceLock::new();
    let after = AFTER.get_or_init(|| Regex::new(r"retry after (\d+)\s*s").expect("regex"));
    let consumed =
        CONSUMED.get_or_init(|| Regex::new(r"consumed \(req/min\): (\d+)").expect("regex"));
    let number = |re: &Regex| re.captures(&lower)?[1].parse::<u64>().ok();
    let wait = number(after).map_or(DEFAULT_WAIT, Duration::from_secs);
    let limit = number(consumed).map(|n| n as usize);
    Some((wait.min(LONGEST_WAIT), limit))
}

impl FetchAdapter for Retrying {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.call(|| self.inner.search(query, limit))
    }
    fn research(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.call(|| self.inner.research(query, limit))
    }
    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError> {
        self.call(|| self.inner.scrape(url))
    }
    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError> {
        self.call(|| self.inner.parse_pdf(path))
    }
    fn spent(&self) -> Spent {
        self.inner.spent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rate_limit_names_its_wait_and_its_limit_and_nothing_else_is_one() {
        let failed = |error: &str| AdapterError::Failed {
            url: "u".into(),
            error: error.into(),
        };
        let beech = "Rate limit exceeded. Consumed (req/min): 11, Remaining (req/min): 0. Upgrade your plan or retry after 9s";
        assert_eq!(
            rate_limited(&failed(beech)),
            Some((Duration::from_secs(9), Some(11)))
        );
        assert_eq!(
            rate_limited(&failed("HTTP 429 Too Many Requests")),
            Some((DEFAULT_WAIT, None))
        );
        assert_eq!(rate_limited(&failed("HTTP 404 Not Found")), None);
        assert_eq!(rate_limited(&AdapterError::Command("x".into())), None);
    }
}
