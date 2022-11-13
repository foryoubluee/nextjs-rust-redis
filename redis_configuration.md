**[If anybody doesn't have RedisJSON installed]**
Step 1 : brew services stop redis
Step 2 : Go to https://github.com/RedisJSON/RedisJSON
Step 3 : Choose a folder to download & Install the following piece of code
Step 4 : git clone https://github.com/RedisJSON/RedisJSON.git
Step 5 : cargo build --release (Make sure you have rust installed or run this command - curl https://sh.rustup.rs -sSf | sh )
Step 6 : redis-server --loadmodule ./your_release_directory_from_installed_RedisJSON/librejson.dylib

**Redis json set command**
JSON.SET movie $ '[{ "title": "Hello world", "description": "blablablablalbalbalb", "ratings": 8.0 },{ "title": "Toy story", "description": "toy story anjay mabar broh", "ratings": 8.9 },{ "title": "Alladin", "description": "Alladin arap apa india sih dia?", "ratings": 8.7 }]'
