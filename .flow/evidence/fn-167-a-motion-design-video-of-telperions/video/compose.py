"""Composes the fn-167 cuts with ffmpeg from the generator renders
(raw/renders), the drawn artifacts (raw/drawn) and the stage dump
(raw/stages): titles, labels, the stage strip and transitions are drawn
here around them. Each clip is encoded on its own, then the clips are joined
with short cross-fades into an H.264 file in the repository's ignored
demo-video/ folder.

Run from the evidence folder once render.py and the drawings have run:
    python3 video/compose.py master|vertical"""
import hashlib
import json
import os
import subprocess
import sys
from dataclasses import dataclass, field

FPS = 24
FADE = 0.3  # seconds each cut overlaps the next
OUT = "../../../demo-video"
CLIPS = "raw/clips"
SANS = "/usr/share/fonts/noto/NotoSans-Regular.ttf"
BOLD = "/usr/share/fonts/noto/NotoSans-Bold.ttf"
MONO = "/usr/share/fonts/Adwaita/AdwaitaMono-Regular.ttf"
WHITE, GREY, ACCENT = "0xF2F2EE", "0xAEB4C0", "0x8FB8FF"
BACKGROUND = "0x22252B"  # the drawn artifacts' background, gamma-encoded


@dataclass
class Text:
    text: str
    x: str
    y: str
    size: int
    colour: str = WHITE
    start: float = 0.0
    end: float = 1e9
    font: str = "sans"  # sans, bold or mono

    def filter(self, duration: float) -> str:
        os.makedirs(f"{CLIPS}/text", exist_ok=True)
        path = f"{CLIPS}/text/{hashlib.sha1(self.text.encode()).hexdigest()[:12]}.txt"
        with open(path, "w") as f:
            f.write(self.text)
        a, b = self.start, min(self.end, duration)
        alpha = (f"if(lt(t,{a}),0,if(lt(t,{a + 0.35}),(t-{a})/0.35,"
                 f"if(lt(t,{b - 0.35}),1,if(lt(t,{b}),({b}-t)/0.35,0))))")
        face = {"sans": f"fontfile={SANS}", "mono": f"fontfile={MONO}",
                "bold": f"fontfile={BOLD}"}[self.font]
        return (f"drawtext={face}:textfile={path}:expansion=none:fontsize={self.size}:"
                f"fontcolor={self.colour}:x={self.x}:y={self.y}:alpha='{alpha}':"
                "line_spacing=8")


@dataclass
class Clip:
    name: str
    seconds: float
    inputs: list[list[str]]
    graph: str  # ends in [base]
    texts: list[Text] = field(default_factory=list)

    def encode(self, cut: str) -> str:
        path = f"{CLIPS}/{cut}/{self.name}.mp4"
        chain = ",".join([t.filter(self.seconds) for t in self.texts] or ["null"])
        graph = f"{self.graph};[base]{chain},format=yuv420p[out]"
        args = ["ffmpeg", "-y", "-v", "error"]
        for i in self.inputs:
            args += i
        args += ["-filter_complex", graph, "-map", "[out]", "-r", str(FPS),
                 "-frames:v", str(round(self.seconds * FPS)), "-c:v", "libx264", "-crf", "12",
                 "-preset", "fast", path]
        subprocess.run(args, check=True)
        return path


def still(path: str, seconds: float) -> list[str]:
    return ["-loop", "1", "-framerate", str(FPS), "-t", str(seconds + 0.5), "-i", path]


def frames(directory: str) -> list[str]:
    return ["-framerate", str(FPS), "-i", f"{directory}/frame-%04d.png"]


def push(i: int, w: int, h: int, seconds: float, zoom: float = 0.07) -> str:
    """A still scaled to cover the frame, pushing in by `zoom` over the clip."""
    grow = f"(1+{zoom}*t/{seconds})"
    return (f"[{i}:v]scale=w='{w}*{grow}':h='{h}*{grow}':eval=frame:"
            f"force_original_aspect_ratio=increase,crop={w}:{h},setsar=1")


def fitted(i: int, w: int, h: int, hold: float = 30.0) -> str:
    """A frame sequence scaled to the frame, its last frame held."""
    return (f"[{i}:v]scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h},"
            f"setsar=1,tpad=stop_mode=clone:stop_duration={hold}")


def band(w: int, h: int, height: int) -> str:
    """A dark band under the captions, darkest at the bottom edge."""
    return (f"drawbox=x=0:y={h - height}:w={w}:h={height // 2}:color=black@0.28:t=fill,"
            f"drawbox=x=0:y={h - height // 2}:w={w}:h={height // 2}:color=black@0.5:t=fill")


def caption(title: str, detail: str, h: int, start: float = 0.15,
            end: float = 1e9, x: str = "80") -> list[Text]:
    return [Text(title, x, str(h - 170), 44, WHITE, start, end, "bold"),
            Text(detail, x, str(h - 108), 28, GREY, start + 0.15, end)]


def section(name: str) -> Text:
    return Text(name, "80", "64", 26, ACCENT, 0.1, font="bold")


def stages() -> dict:
    with open("raw/stages/telperion-1-run2.json") as f:
        return json.load(f)


def thousands(n: float) -> str:
    return f"{n:,.0f}"


# --- the 16:9 master ----------------------------------------------------

W, H = 1920, 1080
STAGES = ["GROW", "PLAN", "EXPAND", "CULL", "DRAW"]


def title_clip(name: str, seconds: float, lines: list[str], w: int, h: int,
               subtitle_y: int) -> Clip:
    title, subtitle = lines
    return Clip(name, seconds, [still("raw/renders/telperion-hero/still.png"
                                      if w > h else "raw/renders/telperion-hero-v/still.png",
                                      seconds)],
                f"{push(0, w, h, seconds, 0.05)},eq=brightness=-0.12:saturation=0.8[base]",
                [Text(title, "(w-text_w)/2", str(subtitle_y - 120), 112 if w > h else 104,
                      WHITE, 0.3, seconds - 0.2, "bold"),
                 Text(subtitle, "(w-text_w)/2", str(subtitle_y), 38, GREY, 0.8, seconds - 0.2)])


def still_clip(name: str, render: str, seconds: float, texts: list[Text],
               zoom: float = 0.07) -> Clip:
    return Clip(name, seconds, [still(f"raw/renders/{render}/still.png", seconds)],
                f"{push(0, W, H, seconds, zoom)},{band(W, H, 260)}[base]", texts)


def strip(active: int, t0: float) -> list[Text]:
    """The stage strip: every stage named in order, the active one lit."""
    texts, x = [], 620
    for i, stage in enumerate(STAGES):
        lit = i == active
        texts.append(Text(stage, str(x), "58", 34, WHITE if lit else "0x6A7080", 0,
                          font="bold" if lit else "sans"))
        if i < len(STAGES) - 1:
            texts.append(Text("\u203a", str(x + len(stage) * 24 + 24), "56", 36, "0x6A7080", 0))
        x += len(stage) * 24 + 80
    return texts


def pipeline_clip(active: int, seconds: float, source: list[str], base: str,
                  lines: tuple[str, str]) -> Clip:
    t0 = active * seconds
    catalogue = f"[1:v]format=rgba,crop=440:760:0:'mod({t0 * 28}+t*28,ih-760)'[cat]"
    graph = (f"{base},{band(W, H, 260)},drawbox=x=0:y=0:w={W}:h=130:color=black@0.45:t=fill"
             f"[bg];{catalogue};[bg][cat]overlay=70:190[base]")
    texts = strip(active, t0) + [
        Text("PARAMETER CATALOGUE", "70", "150", 20, ACCENT, 0, font="bold"),
        Text("rows feed every stage", "70", "960", 20, GREY, 0),
        *caption(*lines, H, x="560"),
    ]
    return Clip(f"pipeline-{active}", seconds, [source, still("raw/catalogue.png", seconds)],
                graph, texts)


def master() -> list[Clip]:
    s = stages()
    ms = s["stagesMs"]
    nodes = len(s["parents"])
    local = nodes - s["crossover"]
    return [
        title_clip("title", 4, ["TELPERION", "a runtime tree generator"], W, H, 600),
        Clip("drift", 5, [frames("raw/renders/telperion-drift")],
             f"{fitted(0, W, H)},{band(W, H, 260)}[base]",
             [section("FIDELITY"), *caption(
                 "Telperion", "seed 1 · 534,778 leaves · 5.48 M wood triangles "
                 "· every one generated", H)]),
        still_clip("bark", "telperion-bark", 2, [section("FIDELITY"), *caption(
            "Close enough to touch", "Telperion's braided trunk: seven lobes, twisting", H)]),
        still_clip("leaf", "telperion-leaf", 2, [section("FIDELITY"), *caption(
            "Down to the leaf", "Telperion's leaf element, built from its rows", H)], 0.04),
        still_clip("palm", "date-palm-crown", 2, [section("FIDELITY"), *caption(
            "Date palm", "fronds, leaflets and the bases they leave on the stem", H)]),
        still_clip("birch", "birch-crown", 2, [section("FIDELITY"), *caption(
            "Silver birch", "hanging twigs, 226,057 leaves", H)]),
        still_clip("beech", "beech-leaf", 1.5, [section("FIDELITY"), *caption(
            "European beech", "its leaf element", H)], 0.04),
        still_clip("oak", "oak-hero", 1.5, [section("FIDELITY"), *caption(
            "Oregon white oak", "125,087 nodes · 715,065 leaves", H)]),
        Clip("grow", 9, [frames("raw/drawn/grow")], f"{fitted(0, W, H)},{band(W, H, 260)}[base]",
             [section("THE GENERATOR"),
              *caption("Space colonization grows the crown",
                       f"{len(s['attractors']) // 3:,} attractors pull every axis; "
                       "each goes out as the wood reaches it", H, 0.2, 5.1),
              *caption("Botanical rules below the crossover",
                       f"{local:,} branches and twigs from the tree's own habit rows",
                       H, 5.3)]),
        Clip("bias", 5, [still("raw/renders/telperion-no-writhe/still.png", 3.5),
                         still("raw/renders/telperion-writhe/still.png", 3.5)],
             f"{push(0, W, H, 3.5, 0.0)}[a];{push(1, W, H, 3.5, 0.0)}[b];"
             f"[a][b]xfade=transition=wiperight:duration=1.6:offset=1.4,{band(W, H, 260)}[base]",
             [section("THE GENERATOR"),
              *caption("One bias field for the supernatural",
                       "Telperion with its writhe at 0, then as shipped at 0.11: "
                       "the field bends every axis", H)]),
        pipeline_clip(0, 4, frames("raw/drawn/skeleton"), f"{fitted(0, W, H)}",
                      ("GROW: the skeleton",
                       f"{nodes:,} nodes · {ms['grow']:.0f} ms")),
        pipeline_clip(1, 4, still("raw/renders/telperion-leaf/still.png", 4),
                      push(0, W, H, 4, 0.03),
                      ("PLAN: the leaf element and the leaf plan",
                       f"{s['plan']['descriptors']:,} twig descriptors · "
                       f"{s['plan']['total']:,} leaves planned · {ms['plan']:.0f} ms")),
        pipeline_clip(2, 4, still("raw/renders/telperion-bare/still.png", 4),
                      push(0, W, H, 4, 0.04),
                      ("EXPAND: rings and the wood surface",
                       f"{s['wood']['vertices']:,} vertices · "
                       f"{s['wood']['triangles']:,} triangles · "
                       f"{ms['rings'] + ms['wood']:.0f} ms")),
        pipeline_clip(3, 4, frames("raw/drawn/leaves"), f"{fitted(0, W, H)}",
                      ("CULL: placed leaves, culled to the crown's shell",
                       f"{s['placed']:,} placed, {s['retained']:,} kept · "
                       f"{ms['placement'] + ms['cull']:.0f} ms")),
        pipeline_clip(4, 4, still("raw/renders/telperion-hero/still.png", 4),
                      push(0, W, H, 4, 0.04),
                      ("DRAW: one tree, six draw calls",
                       "1.8 ms of GPU at 1920x1080 on an RTX 3080")),
        Clip("twist", 6, [frames("raw/renders/telperion-twist")],
             f"{fitted(0, W, H)},{band(W, H, 260)}[base]",
             [section("CONTINUOUS TREE SPACE"),
              *caption("Every parameter is a number",
                       "/surface/twistRate swept live from 0 to 2.4 on Telperion", H)]),
        seeds_clip(),
        consumers_clip(),
        Clip("end", 4, [still("raw/renders/telperion-hero/still.png", 4)],
             f"{push(0, W, H, 4, 0.04)},eq=brightness=-0.14:saturation=0.8[base]",
             [Text("One continuous tree space.", "(w-text_w)/2", "440", 64, WHITE, 0.2, 3.9,
                   "bold"),
              Text("One pipeline.", "(w-text_w)/2", "530", 64, WHITE, 0.7, 3.9, "bold"),
              Text("TELPERION", "(w-text_w)/2", "660", 30, ACCENT, 1.2, 3.9, "bold")]),
    ]


def seeds_clip() -> Clip:
    inputs = [still(f"raw/renders/oak-seed-{n}/still.png", 4) for n in (1, 2, 3, 4)]
    parts = [f"color=c={BACKGROUND}:s={W}x{H}:r={FPS}:d=4.5[bg0]"]
    for n in range(4):
        parts.append(f"[{n}:v]scale=480:540,format=rgba,"
                     f"fade=in:st={0.2 + 0.35 * n}:d=0.4:alpha=1[s{n}]")
    for n in range(4):
        parts.append(f"[bg{n}][s{n}]overlay={n * 480}:250[bg{n + 1}]")
    graph = ";".join(parts) + f";[bg4]null[base]"
    labels = [Text(f"seed {n + 1}", str(n * 480 + 200), "810", 28, GREY, 0.3 + 0.35 * n)
              for n in range(4)]
    return Clip("seeds", 4, inputs, graph,
                [section("CONTINUOUS TREE SPACE"), *labels,
                 Text("The seed picks the specimen", "80", "900", 44, WHITE, 0.2, font="bold"),
                 Text("Oregon white oak, one preset, four seeds", "80", "962", 28, GREY, 0.35)])


def consumers_clip(w: int = W, h: int = H, seconds: float = 12, first: int = 0) -> Clip:
    styles = ["smooth", "blocks", "points"]
    names = ["smooth voxels", "block cubes", "point cloud"]
    inputs = [["-start_number", str(first + 1), *frames(f"raw/drawn/consumers-{s}")]
              for s in styles]
    tall = h > w
    scale = 0.55 if tall else 0.85
    pw, ph = round(620 * scale), round(940 * scale)
    gap = (w - 3 * pw) // 4
    y = 640 if tall else 100
    parts = [f"color=c={BACKGROUND}:s={w}x{h}:r={FPS}:d={seconds + 0.5}[bg0]"]
    for n in range(3):
        parts.append(f"[{n}:v]scale={pw}:{ph},setsar=1[p{n}]")
        parts.append(f"[bg{n}][p{n}]overlay={gap + n * (pw + gap)}:{y}[bg{n + 1}]")
    graph = ";".join(parts) + ";[bg3]null[base]"
    label_y = y + ph + 6
    labels = [Text(names[n], f"{gap + n * (pw + gap) + pw // 2}-text_w/2", str(label_y),
                   24 if tall else 30, GREY, 0.3) for n in range(3)]
    if tall:
        texts = [Text("One tree, many consumers", "(w-text_w)/2", "380", 56, WHITE, 0.1,
                      font="bold"),
                 Text("drawn from telperion/field queries", "(w-text_w)/2", "460", 32, GREY,
                      0.2)]
    else:
        texts = [section("ONE TREE, MANY CONSUMERS"),
                 Text("The same oak's field, three readings", "80", str(h - 110), 40, WHITE,
                      0.2, font="bold"),
                 Text("each built from telperion/field's occupancy queries; no mesh is made",
                      "80", str(h - 58), 26, GREY, 0.35)]
    return Clip("consumers", seconds, inputs, graph, labels + texts)


# --- the 9:16 cut -------------------------------------------------------

VW, VH = 1080, 1920


def vertical() -> list[Clip]:
    s = stages()
    def cap(title: str, detail: str) -> list[Text]:
        return [Text(title, "(w-text_w)/2", str(VH - 330), 52, WHITE, 0.15, font="bold"),
                Text(detail, "(w-text_w)/2", str(VH - 255), 30, GREY, 0.3)]
    vband = band(VW, VH, 420)
    return [
        title_clip("title", 2.5, ["TELPERION", "a runtime tree generator"], VW, VH, 620),
        Clip("drift", 3, [frames("raw/renders/telperion-drift-v")],
             f"{fitted(0, VW, VH)},{vband}[base]",
             cap("Every leaf generated", "Telperion · 534,778 leaves")),
        Clip("grow", 4, [frames("raw/drawn/grow-v")], f"{fitted(0, VW, VH)},{vband}[base]",
             cap("Space colonization",
                 f"{len(s['attractors']) // 3:,} attractors grow the crown")),
        Clip("twist", 3.5, [frames("raw/renders/telperion-twist-v")],
             f"{fitted(0, VW, VH)},{vband}[base]",
             cap("One continuous tree space", "twistRate swept live, 0 to 2.4")),
        consumers_clip(VW, VH, 3, 108),
        Clip("end", 2, [still("raw/renders/telperion-hero-v/still.png", 2)],
             f"{push(0, VW, VH, 2, 0.03)},eq=brightness=-0.14:saturation=0.8[base]",
             [Text("One tree space.", "(w-text_w)/2", "760", 64, WHITE, 0.15, 1.95, "bold"),
              Text("One pipeline.", "(w-text_w)/2", "850", 64, WHITE, 0.35, 1.95, "bold"),
              Text("TELPERION", "(w-text_w)/2", "980", 30, ACCENT, 0.6, 1.95, "bold")]),
    ]


# --- assembly -----------------------------------------------------------

def catalogue() -> None:
    """The catalogue column: every row path in docs/parameters.md, twice over
    so the scroll wraps without a seam."""
    with open("../../../docs/parameters.md") as f:
        rows = [line[5:].strip().strip("`") for line in f if line.startswith("### `")]
    with open(f"{CLIPS}/text/catalogue.txt", "w") as f:
        f.write("\n".join(rows + rows))
    subprocess.run(["magick", "-background", "none", "-fill", "#8A92A3", "-font", MONO,
                    "-pointsize", "19", "-interline-spacing", "9",
                    f"label:@{CLIPS}/text/catalogue.txt", "-gravity", "west",
                    "-extent", "440x", "raw/catalogue.png"], check=True)


def join(paths: list[str], seconds: list[float], out: str) -> None:
    args = ["ffmpeg", "-y", "-v", "error"]
    for p in paths:
        args += ["-i", p]
    parts, last, offset = [], "[0:v]", 0.0
    for i in range(1, len(paths)):
        offset += seconds[i - 1] - FADE
        parts.append(f"{last}[{i}:v]xfade=transition=fade:duration={FADE}:offset={offset:.3f}"
                     f"[x{i}]")
        last = f"[x{i}]"
    parts.append(f"{last}format=yuv420p[out]")
    args += ["-filter_complex", ";".join(parts), "-map", "[out]", "-c:v", "libx264",
             "-preset", "slow", "-crf", "18", "-profile:v", "high", "-movflags", "+faststart",
             "-r", str(FPS), "-an", out]
    subprocess.run(args, check=True)


def main() -> None:
    cut = sys.argv[1] if len(sys.argv) > 1 else ""
    if cut not in ("master", "vertical"):
        sys.exit("usage: compose.py master|vertical")
    os.makedirs(f"{CLIPS}/{cut}", exist_ok=True)
    os.makedirs(f"{CLIPS}/text", exist_ok=True)
    os.makedirs(OUT, exist_ok=True)
    if cut == "master":
        catalogue()
    clips = master() if cut == "master" else vertical()
    paths = []
    for clip in clips:
        paths.append(clip.encode(cut))
        print(f"{cut}/{clip.name}: {clip.seconds} s")
    out = f"{OUT}/telperion-{'master-16x9' if cut == 'master' else 'social-9x16'}.mp4"
    join(paths, [c.seconds for c in clips], out)
    total = sum(c.seconds for c in clips) - FADE * (len(clips) - 1)
    print(f"{out}: {total:.2f} s")


if __name__ == "__main__":
    main()
