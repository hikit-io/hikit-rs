# Features

| Platform | QPS  | MAX_TOKEN | InvalidToken | Body |
|----------|------|-----------|--------------|------|
| Apns     |      |           |              |      |
| Fcm      | 500  |           |              |      |
| Xiaomi   | 500  | 1000      |              |      |
| Huawei   | 6000 | 1000      |              | 4kb  |
| Email    |      |           |              |      |
| Wecom    |      | 1000      |              |      |

# Getting start

```rust

```

# Example

`MONGO_URI=mongodb://root:12334@127.0.0.1:123 cargo run --example simple`

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
|       | fcm | apns | email | wecom | xiaomi | huawei |
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
| fcm        | apns         | email           | xiaomi      | huawei | wecom | |


```

HTTP/1.1 200 OK
Server: nginx
Date: Mon, 24 Oct 2022 03:21:59 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 14571
Connection: close
x-request-id: 4f411329aa9801418c4a2a9bb931f339

HTTP/1.1 200 OK
Server: nginx
Date: Mon, 24 Oct 2022 03:43:07 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 15033
Connection: close
x-request-id: ae882229f223843a5088ce33cfd34674

[{"id":43,"email":"chenwei@agora.io","status":0,"updatedAt":"2022-08-31T09:55:32.000Z"},{"id":61,"email":"gaozehua@agora.io","status":0,"updatedAt":"2022-08-26T10:15:06.000Z"},{"id":64,"email":"lijiali@agora.io","status":0,"updatedAt":"2022-08-31T09:57:38.000Z"},{"id":88,"email":"xiatian@agora.io","status":0,"updatedAt":"2022-09-13T09:19:05.000Z"},{"id":172,"email":"xuchunyu@agora.io","status":0,"updatedAt":"2022-10-24T01:54:40.000Z"},{"id":195,"email":"niezhengxue@agora.io","status":0,"updatedAt":"2022-09-14T10:52:49.000Z"},{"id":207,"email":"wangyingying@agora.io","status":0,"updatedAt":"2022-08-30T09:54:03.000Z"},{"id":237,"email":"denghui@agora.io","status":0,"updatedAt":"2022-10-24T02:54:40.000Z"},{"id":276,"email":"yaozheyun@agora.io","status":0,"updatedAt":"2022-09-06T09:45:00.000Z"},{"id":315,"email":"varun@agora.io","status":0,"updatedAt":"2022-10-17T13:36:09.000Z"},{"id":375,"email":"hermes@agora.io","status":0,"updatedAt":"2022-08-31T23:58:00.000Z"},{"id":402,"email":"william.du@agora.io","status":0,"updatedAt":"2022-08-31T23:40:37.000Z"},{"id":581,"email":"guanziming@agora.io","status":0,"updatedAt":"2022-09-26T08:11:24.000Z"},{"id":599,"email":"lvminbo@agora.io","status":0,"updatedAt":"2022-09-05T07:42:24.000Z"},{"id":602,"email":"melanie@agora.io","status":0,"updatedAt":"2022-08-31T23:42:16.000Z"},{"id":643,"email":"lifan@agora.io","status":0,"updatedAt":"2022-10-24T02:59:15.000Z"},{"id":675,"email":"sunyong@agora.io","status":0,"updatedAt":"2022-10-24T02:49:49.000Z"},{"id":679,"email":"yangchen@agora.io","status":0,"updatedAt":"2022-10-18T04:01:03.000Z"},{"id":725,"email":"wuhaiyang@agora.io","status":0,"updatedAt":"2022-09-21T10:30:59.000Z"},{"id":736,"email":"liuting@agora.io","status":0,"updatedAt":"2022-09-05T07:55:47.000Z"},{"id":749,"email":"zhangchi02@agora.io","status":0,"updatedAt":"2022-08-26T10:15:40.000Z"},{"id":799,"email":"dongfang@agora.io","status":0,"updatedAt":"2022-10-24T02:59:54.000Z"},{"id":830,"email":"melissa@agora.io","status":0,"updatedAt":"2022-10-17T18:42:39.000Z"},{"id":882,"email":"luoqiyong@agora.io","status":0,"updatedAt":"2022-08-25T04:06:45.000Z"},{"id":913,"email":"wanglu@agora.io","status":0,"updatedAt":"2022-09-29T09:07:03.000Z"},{"id":914,"email":"daishang@agora.io","status":0,"updatedAt":"2022-09-09T10:42:58.000Z"},{"id":923,"email":"zhujian@agora.io","status":0,"updatedAt":"2022-10-24T02:02:30.000Z"},{"id":928,"email":"wuqi@agora.io","status":0,"updatedAt":"2022-10-20T06:20:45.000Z"},{"id":975,"email":"zhulu@agora.io","status":0,"updatedAt":"2022-09-21T10:30:00.000Z"},{"id":1067,"email":"shiyang@agora.io","status":0,"updatedAt":"2022-10-17T11:21:48.000Z"},{"id":1075,"email":"hefei@agora.io","status":0,"updatedAt":"2022-08-31T10:00:14.000Z"},{"id":1079,"email":"wangzhaoxu@agora.io","status":0,"updatedAt":"2022-10-24T01:58:16.000Z"},{"id":1103,"email":"liuyuanyuan@agora.io","status":0,"updatedAt":"2022-10-24T02:43:46.000Z"},{"id":1150,"email":"jennifer.phamle@agora.io","status":0,"updatedAt":"2022-10-14T02:56:07.000Z"},{"id":1171,"email":"xiaochuran@agora.io","status":0,"updatedAt":"2022-10-24T03:31:23.000Z"},{"id":1177,"email":"yuyixing@agora.io","status":0,"updatedAt":"2022-09-23T07:06:23.000Z"},{"id":1193,"email":"zhaoyisheng@agora.io","status":0,"updatedAt":"2022-09-29T09:07:58.000Z"},{"id":1237,"email":"dingzhixiang@agora.io","status":0,"updatedAt":"2022-10-12T09:28:42.000Z"},{"id":1239,"email":"ed.brakus@agora.io","status":0,"updatedAt":"2022-09-15T21:41:54.000Z"},{"id":1248,"email":"sunxianbin@agora.io","status":0,"updatedAt":"2022-10-24T02:48:47.000Z"},{"id":1258,"email":"zhaiyuan@agora.io","status":0,"updatedAt":"2022-09-16T05:46:48.000Z"},{"id":1265,"email":"russ@agora.io","status":0,"updatedAt":"2022-10-17T16:30:10.000Z"},{"id":1275,"email":"wangbin@agora.io","status":0,"updatedAt":"2022-08-25T05:01:43.000Z"},{"id":1337,"email":"liling@agora.io","status":0,"updatedAt":"2022-10-19T13:08:31.000Z"},{"id":1342,"email":"yushangrong@agora.io","status":0,"updatedAt":"2022-10-24T03:14:37.000Z"},{"id":1382,"email":"chenyanhong@agora.io","status":0,"updatedAt":"2022-10-24T03:10:29.000Z"},{"id":1396,"email":"guoqi@agora.io","status":0,"updatedAt":"2022-09-22T10:44:54.000Z"},{"id":1408,"email":"miaojian@agora.io","status":0,"updatedAt":"2022-10-24T03:15:24.000Z"},{"id":1446,"email":"yangmingtian@agora.io","status":0,"updatedAt":"2022-10-20T06:20:02.000Z"},{"id":1471,"email":"zhouqing@agora.io","status":0,"updatedAt":"2022-10-24T02:16:50.000Z"},{"id":1473,"email":"guxusheng@agora.io","status":0,"updatedAt":"2022-09-28T12:33:54.000Z"},{"id":1486,"email":"tanya@agora.io","status":0,"updatedAt":"2022-10-14T02:56:53.000Z"},{"id":1503,"email":"changxiaoxiao@agora.io","status":0,"updatedAt":"2022-10-24T02:07:42.000Z"},{"id":1519,"email":"sunwenbin@agora.io","status":0,"updatedAt":"2022-08-25T05:03:18.000Z"},{"id":1603,"email":"liwenlan@agora.io","status":0,"updatedAt":"2022-10-24T02:15:57.000Z"},{"id":1614,"email":"wuchangwei@agora.io","status":0,"updatedAt":"2022-10-24T03:35:29.000Z"},{"id":1617,"email":"kuangwenxiang@agora.io","status":0,"updatedAt":"2022-10-20T06:17:56.000Z"},{"id":1620,"email":"pankaiyun@agora.io","status":0,"updatedAt":"2022-10-24T02:35:50.000Z"},{"id":1634,"email":"wangjunyao@agora.io","status":0,"updatedAt":"2022-10-24T02:42:57.000Z"},{"id":1655,"email":"zhangzhe@agora.io","status":0,"updatedAt":"2022-10-19T13:08:16.000Z"},{"id":1679,"email":"anthony@agora.io","status":0,"updatedAt":"2022-10-17T18:45:56.000Z"},{"id":1688,"email":"sunyifei@agora.io","status":0,"updatedAt":"2022-08-25T06:19:31.000Z"},{"id":1696,"email":"muwencheng@agora.io","status":0,"updatedAt":"2022-10-14T11:17:57.000Z"},{"id":1751,"email":"gaiqingqing@agora.io","status":0,"updatedAt":"2022-10-24T02:46:06.000Z"},{"id":1761,"email":"chenshi@agora.io","status":0,"updatedAt":"2022-10-24T02:47:56.000Z"},{"id":1766,"email":"fuyiyang@agora.io","status":0,"updatedAt":"2022-10-14T11:16:38.000Z"},{"id":1788,"email":"yangting02@agora.io","status":0,"updatedAt":"2022-10-24T02:40:18.000Z"},{"id":1795,"email":"chenzhipeng02@agora.io","status":0,"updatedAt":"2022-10-24T02:29:18.000Z"},{"id":1804,"email":"xieyuqi@agora.io","status":0,"updatedAt":"2022-10-24T02:09:16.000Z"},{"id":1823,"email":"yanzhiqiang@agora.io","status":0,"updatedAt":"2022-10-24T03:07:24.000Z"},{"id":1834,"email":"zhaoqingya@agora.io","status":0,"updatedAt":"2022-10-24T02:23:48.000Z"},{"id":1838,"email":"yanguangfei@agora.io","status":0,"updatedAt":"2022-10-24T02:44:43.000Z"},{"id":1850,"email":"mingshiqiang@agora.io","status":0,"updatedAt":"2022-10-24T02:04:37.000Z"},{"id":1855,"email":"liufengbo@agora.io","status":0,"updatedAt":"2022-10-24T03:08:06.000Z"},{"id":1879,"email":"huangzijian@agora.io","status":0,"updatedAt":"2022-10-24T03:18:08.000Z"},{"id":1884,"email":"hebianyu@agora.io","status":0,"updatedAt":"2022-09-01T10:59:26.000Z"},{"id":1885,"email":"qihongbiao@agora.io","status":0,"updatedAt":"2022-09-23T10:18:00.000Z"},{"id":1905,"email":"rafael@agora.io","status":0,"updatedAt":"2022-10-17T15:07:52.000Z"},{"id":1911,"email":"lijiaxuan@agora.io","status":0,"updatedAt":"2022-08-25T05:10:54.000Z"},{"id":1934,"email":"qiaolibiao@agora.io","status":0,"updatedAt":"2022-10-24T03:15:55.000Z"},{"id":1941,"email":"laomengan@agora.io","status":0,"updatedAt":"2022-10-24T03:34:40.000Z"},{"id":1945,"email":"guowenda@agora.io","status":0,"updatedAt":"2022-09-23T10:18:52.000Z"},{"id":1947,"email":"wendy.s.yao@agora.io","status":0,"updatedAt":"2022-08-25T06:19:55.000Z"},{"id":1948,"email":"jane.jw.shi@agora.io","status":0,"updatedAt":"2022-08-25T06:20:19.000Z"},{"id":1961,"email":"angela@agora.io","status":0,"updatedAt":"2022-10-17T15:08:03.000Z"},{"id":1973,"email":"dengyuanqin@agora.io","status":0,"updatedAt":"2022-10-24T03:03:53.000Z"},{"id":1974,"email":"mupeng@agora.io","status":0,"updatedAt":"2022-10-24T02:08:27.000Z"},{"id":1981,"email":"sasha@agora.io","status":0,"updatedAt":"2022-10-17T13:36:17.000Z"},{"id":1982,"email":"adnan@agora.io","status":0,"updatedAt":"2022-10-17T18:42:07.000Z"},{"id":1983,"email":"john@agora.io","status":0,"updatedAt":"2022-10-17T13:36:26.000Z"},{"id":1984,"email":"karen.klemp@agora.io","status":0,"updatedAt":"2022-10-17T17:21:45.000Z"},{"id":1993,"email":"chenshi02@agora.io","status":0,"updatedAt":"2022-09-30T10:05:42.000Z"},{"id":2003,"email":"dongmingze@agora.io","status":0,"updatedAt":"2022-10-19T08:29:16.000Z"},{"id":2011,"email":"zhouxiangtong@agora.io","status":0,"updatedAt":"2022-08-25T06:45:21.000Z"},{"id":2016,"email":"zhangfujie@agora.io","status":0,"updatedAt":"2022-10-24T03:04:38.000Z"},{"id":2017,"email":"hewei@agora.io","status":0,"updatedAt":"2022-08-25T05:13:56.000Z"},{"id":2069,"email":"maaz.rahmani@agora.io","status":0,"updatedAt":"2022-10-14T06:50:33.000Z"},{"id":2091,"email":"zhengfeng@agora.io","status":0,"updatedAt":"2022-09-09T10:43:28.000Z"},{"id":2094,"email":"xiaoyang@agora.io","status":0,"updatedAt":"2022-10-19T13:17:39.000Z"},{"id":2096,"email":"eric.york@agora.io","status":0,"updatedAt":"2022-10-17T17:41:30.000Z"},{"id":2097,"email":"chris.delgado@agora.io","status":0,"updatedAt":"2022-10-17T18:51:00.000Z"},{"id":2124,"email":"anthony.smith@agora.io","status":0,"updatedAt":"2022-10-17T16:30:20.000Z"},{"id":2125,"email":"stefanie.phillips@agora.io","status":0,"updatedAt":"2022-10-17T16:44:02.000Z"},{"id":2133,"email":"xuqinxin@agora.io","status":0,"updatedAt":"2022-10-24T03:19:50.000Z"},{"id":2139,"email":"chenshiyi@agora.io","status":0,"updatedAt":"2022-09-09T10:43:19.000Z"},{"id":2140,"email":"fanyuanyuan02@agora.io","status":0,"updatedAt":"2022-10-24T03:03:16.000Z"},{"id":2162,"email":"zhaojiuzeng@agora.io","status":0,"updatedAt":"2022-08-25T05:16:15.000Z"},{"id":2172,"email":"zhangshuncheng@agora.io","status":0,"updatedAt":"2022-10-24T03:36:12.000Z"},{"id":2180,"email":"wangchen02@agora.io","status":0,"updatedAt":"2022-10-19T08:29:28.000Z"},{"id":2194,"email":"guoxiaoying@agora.io","status":0,"updatedAt":"2022-10-20T06:20:32.000Z"},{"id":2195,"email":"wenglinsheng@agora.io","status":0,"updatedAt":"2022-10-24T02:22:39.000Z"},{"id":2200,"email":"zoujiaming@agora.io","status":0,"updatedAt":"2022-09-20T08:59:19.000Z"},{"id":2202,"email":"geshaoshun@agora.io","status":0,"updatedAt":"2022-09-07T07:05:30.000Z"},{"id":2217,"email":"changzhenkang@agora.io","status":0,"updatedAt":"2022-10-20T06:20:56.000Z"},{"id":2222,"email":"aaron.brenes@agora.io","status":0,"updatedAt":"2022-10-17T16:30:29.000Z"},{"id":2223,"email":"mark.paniagua@agora.io","status":0,"updatedAt":"2022-10-17T18:42:25.000Z"},{"id":2233,"email":"qinqijin@agora.io","status":0,"updatedAt":"2022-09-28T12:34:06.000Z"},{"id":2253,"email":"chengdongbin@agora.io","status":0,"updatedAt":"2022-09-16T06:23:54.000Z"},{"id":2254,"email":"luoyonghao@agora.io","status":0,"updatedAt":"2022-08-25T05:20:13.000Z"},{"id":2258,"email":"heather.lowe@agora.io","status":0,"updatedAt":"2022-10-17T15:13:48.000Z"},{"id":2260,"email":"leo.an@agora.io","status":0,"updatedAt":"2022-10-17T17:41:11.000Z"},{"id":2266,"email":"xuyutong@agora.io","status":0,"updatedAt":"2022-09-23T10:18:40.000Z"},{"id":2276,"email":"luosihong@agora.io","status":0,"updatedAt":"2022-08-25T05:25:53.000Z"},{"id":2278,"email":"yaoshiyi@agora.io","status":0,"updatedAt":"2022-09-29T09:06:53.000Z"},{"id":2284,"email":"rick.michael@agora.io","status":0,"updatedAt":"2022-10-17T16:09:12.000Z"},{"id":2285,"email":"kyle.bellows@agora.io","status":0,"updatedAt":"2022-10-17T16:43:52.000Z"},{"id":2292,"email":"jiangzhilin@agora.io","status":0,"updatedAt":"2022-10-24T02:58:35.000Z"},{"id":2304,"email":"lixiang03@agora.io","status":0,"updatedAt":"2022-10-24T03:16:32.000Z"},{"id":2305,"email":"yangmeng@agora.io","status":0,"updatedAt":"2022-09-14T10:52:40.000Z"},{"id":2314,"email":"yuanjia@agora.io","status":0,"updatedAt":"2022-08-29T09:53:59.000Z"},{"id":2323,"email":"samuel.akpo@agora.io","status":0,"updatedAt":"2022-10-17T13:29:21.000Z"},{"id":2333,"email":"navame.zaiani@agora.io","status":0,"updatedAt":"2022-10-17T15:08:14.000Z"},{"id":2335,"email":"guopengdong@agora.io","status":0,"updatedAt":"2022-10-24T02:31:35.000Z"},{"id":2336,"email":"yiyufeng@agora.io","status":0,"updatedAt":"2022-09-23T10:18:12.000Z"},{"id":2338,"email":"weiyunteng@agora.io","status":0,"updatedAt":"2022-08-26T10:15:56.000Z"},{"id":2352,"email":"guopenglong@agora.io","status":0,"updatedAt":"2022-10-24T02:31:26.000Z"},{"id":2361,"email":"chenyuchen@agora.io","status":0,"updatedAt":"2022-09-09T10:43:09.000Z"},{"id":2362,"email":"wangmengxiao@agora.io","status":0,"updatedAt":"2022-08-26T10:16:41.000Z"},{"id":2363,"email":"yuanxinyi@agora.io","status":0,"updatedAt":"2022-08-25T05:26:53.000Z"},{"id":2366,"email":"xuliangliang@agora.io","status":0,"updatedAt":"2022-09-21T10:36:14.000Z"},{"id":2368,"email":"wangxiting@agora.io","status":0,"updatedAt":"2022-09-30T10:06:01.000Z"},{"id":2372,"email":"chenyamin@agora.io","status":0,"updatedAt":"2022-10-24T03:36:46.000Z"},{"id":2384,"email":"caihaoyu@agora.io","status":0,"updatedAt":"2022-10-24T02:34:05.000Z"},{"id":2390,"email":"corey.brown@agora.io","status":0,"updatedAt":"2022-10-17T18:42:16.000Z"},{"id":2391,"email":"ganyuqing@agora.io","status":0,"updatedAt":"2022-10-14T11:18:54.000Z"},{"id":2393,"email":"lijiayi02@agora.io","status":0,"updatedAt":"2022-09-30T10:06:39.000Z"},{"id":2394,"email":"yueqiwen@agora.io","status":0,"updatedAt":"2022-09-30T10:05:23.000Z"},{"id":2396,"email":"lishiju@agora.io","status":0,"updatedAt":"2022-10-24T02:01:12.000Z"},{"id":2398,"email":"mengxuanyi@agora.io","status":0,"updatedAt":"2022-10-09T09:30:53.000Z"},{"id":2403,"email":"alexanderliang@agora.io","status":0,"updatedAt":"2022-08-26T10:12:57.000Z"},{"id":2404,"email":"liwei02@agora.io","status":0,"updatedAt":"2022-10-24T02:03:47.000Z"},{"id":2406,"email":"huyifeng@agora.io","status":0,"updatedAt":"2022-09-23T03:49:12.000Z"},{"id":2415,"email":"chenhang@agora.io","status":0,"updatedAt":"2022-10-21T10:33:34.000Z"},{"id":2417,"email":"robert.connors@agora.io","status":0,"updatedAt":"2022-10-20T21:00:52.000Z"},{"id":2419,"email":"wangjin@agora.io","status":0,"updatedAt":"2022-10-24T02:22:01.000Z"},{"id":2430,"email":"tianzhilei@agora.io","status":0,"updatedAt":"2022-10-14T11:17:47.000Z"},{"id":2432,"email":"chenshu@agora.io","status":0,"updatedAt":"2022-09-22T10:44:32.000Z"},{"id":2433,"email":"yuanjunjie@agora.io","status":0,"updatedAt":"2022-10-20T10:24:00.000Z"},{"id":2437,"email":"Tyler.Pak@agora.io","status":0,"updatedAt":"2022-10-17T20:34:28.000Z"},{"id":2440,"email":"qinhaifeng@agora.io","status":0,"updatedAt":"2022-10-20T06:33:57.000Z"},{"id":2442,"email":"duanshaoshuai@agora.io","status":0,"updatedAt":"2022-10-20T10:24:14.000Z"},{"id":2447,"email":"xuyu@agora.io","status":0,"updatedAt":"2022-10-21T10:32:02.000Z"},{"id":2452,"email":"liuliang@agora.io","status":0,"updatedAt":"2022-10-24T02:18:31.000Z"},{"id":2458,"email":"kevinmichelrabefaritra@agora.io","status":0,"updatedAt":"2022-09-04T14:04:19.000Z"},{"id":2460,"email":"huangyongbin@agora.io","status":0,"updatedAt":"2022-09-07T06:59:41.000Z"}]