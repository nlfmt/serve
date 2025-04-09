cd app
pnpm build
cd ..

# need to allow dirty so build files can be included in publish
cargo publish --allow-dirty