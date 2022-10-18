# Features

| Platform | QPS  | MAX_TOKEN | InvalidToken | Body |
|----------|------|-----------|--------------|------|
| Apns     |      |           |              |      |
| Fcm      | 500  |           |              |      |
| Xiaomi   | 500  | 1000      |              |      |
| Huawei   | 6000 | 1000      |              | 4kb  |
| Email    |      |           |              |      |
| Wecom    |      | 1000      |              |      |
| AgoraRtm |      |           |              |      |

# Arch

### Layer

platform: fcm, apns, email, wecom, huawei, xiaomi

lib:

db: mysql, mongo

service:

interface: http, grpc, graphql

```text
 ---------------------------------------------------------------------
| interface |  
|       -------------------------         
|       | grpc | http | graphql |
|       -------------------------
|
|---------------------------------------------------------------------
| service | App{ db: MysqlOrMongo, svc: lib::Service}
|        ------------------------------
|       | register_token| push_message | 
|        ------------------------------
|----------------------------------------------------------------------
| db |  mysql | mongodb |
|       ---------------------------------------------------------
|       | insert_token | fetch_tokens | fetch_chans | fetch_app |
|       ---------------------------------------------------------
|-----------------------------------------------------------------------
| lib | Service: { pushers:Vec<Pusher> }
|       --------------------------------------------------
|       | register_client | remove_client | push_message |
|       --------------------------------------------------
|
|-----------------------------------------------------------------------
| platform |
|       ------------------------------------------------------
|       | fcm | apns | email | wecom | xiaomi | huawei | rtm |
|       ------------------------------------------------------
|
|

 ------------------------------------------------------------------------

|            |              |                 |             |        |       |           |
| ---------- | ------------ | --------------- | ----------- | ------ | ----- | --------- |
| http       | grpc         | graphql         |
| service    |              | init            |
| db_service | insert_token | delete_token    | insert_chan |
| lib        | push_message | register_client |
| fcm        | apns         | email           | xiaomi      | huawei | wecom | agora-rtm |


```
