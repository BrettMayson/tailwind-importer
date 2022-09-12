# Yew Tailwind

## For Applications

Create a tailwind.css for your application, including all compatible crates.

Requires `tailwindcss` to be in the path, recommended to use `npm i -g tailwindcss`.

**build.rs**
```rust
fn main() {
    tailwind_importer::build_frontend();
}
```

## For Libraries

This will create a copy of the source code in the target out directory so it can be read by the tailwind command.

**build.rs**
```rust
fn main() {
    tailwind_importer::build_library();
}
```
