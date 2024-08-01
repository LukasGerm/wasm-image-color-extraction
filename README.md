# Getting Started

## Installation

Run `yarn add wasm-image-color-extraction`

## Usage

```
        import init,{ extract_color } from 'wasm_image_color_extraction';
        async function extractColor() {
            console.log(await extract_color("https://fastly.picsum.photos/id/13/2500/1667.jpg?hmac=SoX9UoHhN8HyklRA4A3vcCWJMVtiBXUg0W4ljWTor7s"))
        }
        init().then(() => {

            document.getElementById("button").addEventListener("click", extractColor);
        })

```

## Development

### Prerequisites

1. Have `wasm-pack` installed

### build

Either run `wasm-pack build --target web` manually or go to `./example` and run `yarn build`

### dev

Run `yarn dev` in `example` folder.

### publish

1. `wasm-pack login`
2. `wasm-pack build --target web`
3. `wasm-pack publish`
