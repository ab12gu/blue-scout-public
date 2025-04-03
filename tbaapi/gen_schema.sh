python3 get_api_schema.py
rm -rf /tmp/tbaapi/
openapi-generator generate -i api_v3.json -g rust -o /tmp/tbaapi/
rm -rf src/
cp -r /tmp/tbaapi/src/ src/
