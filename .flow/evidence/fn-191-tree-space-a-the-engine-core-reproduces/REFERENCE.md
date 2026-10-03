# fn-191 R4: the whole-tree 3D reference for later phases

**Choice.** The runnable reference is L-Py (`openalea/lpy`, commit `4226991`, 2026-06-12), in particular its shipped Hallé–Oldeman architectural models: `models/ArchiModels/{leuwenberg,schoute,massart,cook,nozeran}.lpy`. Pałubicki et al. 2009's published figures stand beside it as a mature-tree target that can only be looked at, not run. Later phases run L-Py unchanged and compare against what it grows; nothing of it is copied.

## Reasons (measured on 2026-10-03 unless marked read)

1. **It runs unchanged, offline and headless.** It installed without root from the `openalea3` channel (`micromamba create -c openalea3 -c conda-forge openalea.lpy`) in 22 s, giving lpy 3.15.4 and PlantGL 3.23.2 on Python 3.13. With `QT_QPA_PLATFORM=offscreen`, `Lsystem(file).derive()` grew each model in 0.01 to 0.34 s. Parameters can be overridden through the `Lsystem(file, {name: value})` constructor without editing a file (read).
2. **Its licence permits running.** The README states CeCILL, compatible with the GNU GPL (read). One anomaly: the repository's `LICENSE.txt` is empty, and GitHub reports `NOASSERTION`. Running it is all we do.
3. **It is grounded in botany.** The ArchiModels files are textbook architectural models (Hallé, Oldeman and Tomlinson 1978), the frame the tree space's coverage claim is counted in. Its `share/training/15-borchert-honda.lpy` is the resource allocation Pałubicki 2009 uses.
4. **Its outputs compare with ours.** The module string counts internodes and branches by type, and a scene exports to OBJ (leuwenberg 3.0 MB), so counts, orders and bounds can be compared:

   | model | derive | modules | triangles |
   |---|---|---|---|
   | leuwenberg | 0.01 s | 4,841 | 37,120 |
   | schoute | 0.01 s | 3,801 | 21,696 |
   | massart | 0.01 s | 1,087 | 12,376 |
   | 02-houppier-light | 0.34 s | 605 | 40,608 |

5. **Every alternative fails to run.**
   - Pałubicki 2009 published figures only; the paper's L+C code was never released, and a search found no port by the authors (read).
   - MAppleT is Python 2 only, with 22 Python 3 syntax errors, and depends on `vplants` packages that no channel carries.
   - AmapSim, the closest to the reference axis, has no public download.
   - Blender's Sapling (Weber–Penn) runs, but its rules are geometric, not botanical.
   - GroIMP runs on Java, which is not installed here; its tree examples were not checked.

## Limits to carry forward

- L-Py's shipped trees are teaching-scale, from 605 to 4,841 modules. None is a mature tree, and none is grown by physiological age. Pałubicki's mature trees reach about 700,000 metamers (read), which is why its figures stay beside L-Py as the look a mature tree should reach.
- `03 - leuwenberg-light.lpy` fails out of the box (`diffuseInterception() missing 'ghi'`), drift from the current PlantGL API. A light-driven reference for phase E needs a working light model; this is phase E's question.
- The choice is a system-design judgment the host should confirm before phase C relies on it. The worker records it; the host decides.

The install used to measure this, and the driver script, sit in the worker session's scratchpad. They are not in the repository.
