ip=$(ip -o route get to 8.8.8.8 | sed -n 's/.*src \([0-9.]\+\).*/\1/p')
echo "==> using ip $ip"
echo "==> build UX"
(cd ui/ && npm run build)
echo "==> clean up old dist"
rm -rf public/
echo "==> make new public folder"
mkdir -p public
echo "==> copy latest dist"
cp -R ui/dist/. public/.
cross build --release --target x86_64-unknown-linux-musl
echo "==> 👷 lets build the image..."
DOCKER_BUILDKIT=1 docker build -t baqi .
