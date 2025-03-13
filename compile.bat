cargo build --release
cd gui
pip install -r requirements.txt
python -m PyInstaller -F main.py --icon=img/planet.ico
cd ..
mkdir installation
move /y "gui\dist\main.exe" installation
move /y "target\release\rest_api.exe" installation
xcopy "gui\img" "installation\img" /y /e /i
