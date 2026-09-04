/* ------------------------------------------------------------------ *
 * THE ONE SOURCE OF CHANCE
 *
 * Every random number the generator ever draws comes from here, and
 * the whole library is a pure function of the seed because of it. The
 * spec's word is "consistently": the seed varies detail, it does not
 * gamble on the outcome, and that promise is only checkable if there
 * is exactly one place chance enters.
 *
 * SplitMix32. Chosen over the xorshift32 the fn-11.1 placeholder used
 * because xorshift32's low bits are visibly correlated on neighbouring
 * seeds, and the panel has a reroll button - the owner will walk
 * consecutive seeds and must not be able to learn the sequence.
 * Math.random is not a candidate at any price: it is unseeded.
 * ------------------------------------------------------------------ */

export interface Rng {
  /** The next uniform in [0, 1). */
  next(): number;
  /** The next uniform in [min, max). */
  range(min: number, max: number): number;
}

/** A stream from `seed`. Any uint32 is a seed, zero included: the
 *  first thing SplitMix32 does is add the golden-ratio constant, so
 *  there is no degenerate state to special-case. */
export function createRng(seed: number): Rng {
  let state = seed >>> 0;

  const next = (): number => {
    state = (state + 0x9e_37_79_b9) >>> 0;
    let z = state;
    z = Math.imul(z ^ (z >>> 16), 0x21_f0_aa_ad);
    z = Math.imul(z ^ (z >>> 15), 0x73_5a_2d_97);
    z = (z ^ (z >>> 15)) >>> 0;
    // Divide by 2^32, never 2^32 - 1: the result has to exclude 1.
    return z / 0x1_00_00_00_00;
  };

  return {
    next,
    range: (min, max) => min + (max - min) * next(),
  };
}
