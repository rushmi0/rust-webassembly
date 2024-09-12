# Dioxus v0.5 + Tailwindcss + daisyui

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

2. Install Dioxus CLI:

```bash
cargo install dioxus-cli
```

3. Install daisyUI, a plugin for Tailwind CSS:

```bash
npm i -D daisyui@latest
```

4. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

5. Run the following command in the root of the project to start the Dioxus dev server:

```bash
npm run web
```

- Open the browser to http://localhost:8080
- Configure Project: https://dioxuslabs.com/learn/0.5/CLI/configure
- Tailwind css: https://tailwindcss.com/docs/installation
- Daisy ui: https://daisyui.com/docs/install