# Snake

Basic snake clone created in [Bevy](https://bevyengine.org/).

# Producing a wasm Build

The following commands are used to produce the `.wasm` and `.js` files required
for running the game in a browser.

```
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir ./out --out-name snake-bevy .\target\wasm32-unknown-unknown\release\snake-bevy.wasm
wasm-opt -Oz .\out\snake-bevy_bg.wasm -o .\out\snake-bevy_bg.opt.wasm
```

# Known Issues

## INVALID_SUBRESOURCE_STATE Errors in Console 

```
2025-04-07T11:39:07.431624Z ERROR wgpu_hal::auxil::dxgi::exception: ID3D12CommandQueue::ExecuteCommandLists: Using IDXGISwapChain::Present on Command List (0x0000021B977773B0:'Internal DXGI CommandList'): Resource state (0x4: D3D12_RESOURCE_STATE_RENDER_TARGET) of resource (0x0000021B977FEFB0:'Unnamed ID3D12Resource Object') (subresource: 0) is invalid for use as a PRESENT_SOURCE.  Expected State Bits (all): 0x0: D3D12_RESOURCE_STATE_[COMMON|PRESENT], Actual State: 0x4: D3D12_RESOURCE_STATE_RENDER_TARGET, Missing State: 0x0: D3D12_RESOURCE_STATE_[COMMON|PRESENT]. [ EXECUTION ERROR #538: INVALID_SUBRESOURCE_STATE]
```

See below wgpu issue for more details

https://github.com/gfx-rs/wgpu/issues/4247

To work around this issue, set the `WGPU_BACKEND` environment variableto the backend you want to be initialized.

e.g. `WGPU_BACKEND=dx12`
