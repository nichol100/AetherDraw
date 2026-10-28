# Architecture

```mermaid
flowchart TB
  subgraph Platform[Platform Layer]
    FS[Files/Paths]
    WIN[Windowing]
    TAB[Tablet Input]
  end
  subgraph Core[Core]
    DOC[Document]
    LAY[Layer System]
    COMP[Compositor]
    FILT[Filter Pipeline]
  end
  subgraph Engines[Engines]
    BRUSH[Brush Engine]
    ERASE[Eraser]
    ANIM[Animation Timeline]
    AUDIO[Audio Engine + Mixer]
    R3D[3D Viewport]
    PHYS[Physics]
  end
  subgraph UI[UI]
    THEME[Theme System (tokens JSON)]
    PANELS[Dockable Panels]
    WS[Workspaces]
  end
  subgraph IO[IO]
    FMT[File Format (.aether)]
    PLUGIN[Plugin API]
  end
  Platform --> Core
  Core --> Engines
  UI --> Core
  IO --> Core
```

## Modules
- `platform/` - OS-specific code (files, windowing, tablet).
- `canvas/` - document, layers, compositor, viewport.
- `brush/` - brush engine, presets, stabilizer, symmetry.
- `eraser/` - eraser types and settings.
- `filters/` - non-destructive filter layers.
- `animation/` - timeline, keyframes, tweening.
- `audio/` - tracks, clips, mixer, DSP.
- `render3d/` - meshes, camera, materials.
- `physics/` - rigid/soft/cloth/particles.
- `ui/` - theme tokens, panels, workspaces, shortcuts.
- `fileformat/` - .aether zip + manifest.
- `plugin/` - plugin trait.