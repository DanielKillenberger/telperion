# Browser visual checkpoint

Four seed-1 CPU/GPU captures use matching cameras and 1280x720 canvases. The host sees no obvious structural difference in these views. Oak is a dark interior-canopy view; spruce shows branches but does not resolve individual needle contacts. These are limited visual evidence, not complete attachment, motion, seed or device qualification. Owner verdict for these browser views is pending; the native hero verdict remains separately recorded in OWNER-VISUAL.md.

The first attempt produced blank CPU images after an initial canvas-resize race. All four images and provenance are preserved under browser-close-before-resize-settle/. One bounded retry waited two initial animation frames before generation and produced all four nonblank images. No production code was changed for this capture retry.
