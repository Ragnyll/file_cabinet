# Installation
## MongoDb
```
yay -S mongodb-bin
systemctl --user start monogodb.service

```

setup tables
```
mongo
use tagdb
> db.createCollection("docs")
```
