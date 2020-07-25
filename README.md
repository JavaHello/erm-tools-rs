# ERM-TOOLS 

## 使用配置

配置示例:

```json
{
    "diffType": "erm-db", // 比较类型 erm-erm, erm-db, db-db, db-erm 4种模式
    "dbType": "mysql", // 数据库类型，目前只支持 mysql
    "sourceErmFile": { // 基准 erm 文件
        "dbName": "demodb",
        "ermPath": "./erms/",
        "ermFiles": ["db.erm"]
    },
    "targetErmFile": { // 目标 erm 文件
        "dbName": "demodb",
        "ermPath": "./erms/",
        "ermFiles": ["db2.erm"]
    },
    "sourceDb": { // 基准 db 配置
        "dbName": "demodb",
        "dbHost": "127.0.0.1",
        "dbUser": "root",
        "dbPassword": "123456",
        "dbPort": "3306"
    },
    "targetDbList": [{ // 目标 db 配置
        "dbName": "demodb",
        "dbHost": "127.0.0.1",
        "dbUser": "root",
        "dbPassword": "123456",
        "dbPort": "3306"
    }],
    "outPath": "", // 输出目录
    "genDdl": true, // 生成 ddl
    "genMd": false // 生成 md
}

```
    
