# Phases

## Phase 1 - MVP: brushes, erasers, layers, blend modes, themes

### Features
- Brushes: pencil, pen, ink, marker, airbrush, watercolor, oil, charcoal, pastel, texture/stamp, pattern, scatter.
- Custom brush engine: size, opacity, flow, hardness, spacing, angle, jitter, scatter, texture, smoothing.
- Pressure, tilt, velocity sensitivity; stroke stabilizer; symmetry (mirror/radial); presets; import/export brush packs.
- Erasers: hard, soft, textured, stroke, layer-aware, color eraser, smudge, blur.
- Layers: raster, vector, text, 3D, reference, group; blend modes (16); opacity, lock, hide, alpha lock, clipping, masks; filter layers (stubs); undo/redo; drag reorder.
- Themes: 8 built-in + custom editor (tokens JSON).

### Data structures
- `Document { width, height, layers: Vec<Layer>, active_layer }`
- `Layer` enum; `RasterLayer { pixels: Vec<u8> (RGBA), ... }`
- `BrushSettings`, `StrokePoint`, `BrushEngine`.
- `ThemeTokens { colors, radii, spacing }`.

### Key algorithms
- Stamp spacing along a stroke: `spacing_px = size * spacing`.
- Pressure curve: `value * (1 - sens + sens * pressure)`.
- Stroke stabilizer: moving average + lerp.
- Compositor: per-pixel blend + alpha over.
- Symmetry: rotate/mirror point sets around center.

### Test checklist
- Launch on Windows/Linux.
- Tablet pressure changes brush size.
- Save/load project file.
- GPU rendering (canvas shows).
- Theme switching updates UI.
- Export PNG.

## Phase 2 - Animation timeline
Features: cels, keyframes, tweening (pos/scale/rot/opacity), onion skinning, multiple anim layers, camera layer, fps, playback/scrub/loop, export GIF/MP4/PNG seq/sprite sheet.
Data: `Timeline`, `Cel`, `Keyframe`, `Tween`.
Algorithms: linear/bezier interpolation, onion skin tinting.
Tests: playback smooth, export sync.

## Phase 3 - Sound editing
Features: multi-track audio, import WAV/MP3/OGG/FLAC/AIFF, record mic, drag/drop, waveforms, scrub audio, sample-accurate sync, clip edit, fades/envelopes, effects, master bus, mixer, lip-sync, beat detection, sound library, synth, audio-driven animation, physics sound hooks, sync check, undo/redo, audio in project.
Data: `AudioTrack`, `AudioClip`, `Envelope`, `EffectChain`.
Algorithms: resampling, DSP biquads, FFT for beat detection, phoneme detection.
Tests: low-latency playback, recording, sync.

## Phase 4 - 3D viewport
Features: mesh modeling (extrude, bevel, loop cut, subdiv, sculpt), materials, camera, rendering.
Data: `Mesh`, `Material`, `Camera3D`.
Algorithms: subdivision, bevel, tessellation.

## Phase 5 - Physics
Features: rigid/soft/cloth/particles, collisions, gravity/wind, bake.
Data: `PhysicsWorld`, bodies.
Algorithms: impulse solver, spring-mass cloth, particle forces.

## Phase 6 - 3D drawing
Features: grease pencil, 2.5D parallax, sketch-to-3D.
Data: 3D stroke, parallax layer.
Algorithms: stroke-to-mesh, depth sorting.