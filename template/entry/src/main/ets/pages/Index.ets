import { NshSurface } from "@nsh/harmonyos";

// The app fills the window: written in Rust, it is linked into the native
// library (`runApp`); written in TSX, its bundle is a raw file of the HAP.
@Entry
@Component
struct Index {
  private surface: NshSurface = new NshSurface(this.getUIContext());

  aboutToAppear(): void {
    if (this.surface.hasApp) {
      this.surface.runApp();
    } else {
      this.surface.loadResource("bundle.js");
    }
  }

  aboutToDisappear(): void {
    this.surface.dispose();
  }

  onBackPress(): boolean {
    return this.surface.onBackPress();
  }

  build() {
    Column() {
      ContentSlot(this.surface.content)
    }
    .width("100%")
    .height("100%")
  }
}
