set -e
cd "$(dirname "$0")"
cd ..

cd src-ui
npm run build
cd ..
cargo build --release

sudo mkdir -p /opt/gridlaunch
sudo cp target/release/gridlaunch /opt/gridlaunch/gridlaunch
sudo cp src/assets -r /opt/gridlaunch/assets/

sudo chown -R root:root /opt/gridlaunch
sudo chmod -R 755 /opt/gridlaunch






