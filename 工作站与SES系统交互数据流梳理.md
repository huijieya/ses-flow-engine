工作站与SES系统交互数据流梳理
步骤： 连接/登录 -> 作业循环
内容：数据流向、调用的接口以及参数形式。

1. 连接与登录阶段 (Connect & Login)

HTTP 用于请求/响应，SSE 用于实时监听。

A. SSE 长连接建立 (核心关注点)

SSE 连接通常在应用启动或登录成功后立即建立，用于接收心跳、AGV到达、波次结束等实时事件。

- 调用方法: [WcsSSE.Connect()]
- 请求方式: HTTP POST (注意：标准SSE通常是GET，但这里实现为POST以携带鉴权或站点信息)。
- URL: [WcsSSEUrl](例如: http://192.168.110.206:12300/station/operation/connect)
- Header:
  - Accept: text/event-stream
  - Accept-Language: zh-CN
  - requestId: 随机生成的 GUID
- Body (JSON):
{
  "ClientId": "nlwW-3g8", // 来自配置的 LeftClientID 或 RightClientID
  "PlatformId": "5Z7fVd...",
  "StationIds": ["nlwW-3g8"]
}
- 返回值: 不是一个传统的 JSON 响应，而是一个持续打开的数据流。
- 消息处理 ([OnMessageReceived]:
客户端会逐行读取服务器推送的数据，格式通常为 data: {JSON字符串}。
  接收到的典型消息类型 (messageType):
  1. Heart_Beat (心跳):
    - 内容: { "RcsStatus": "ONLINE", "StationList": [...] }
    - 作用: 更新 RCS (机器人控制系统) 在线状态。
  2. Agv_Arrived (AGV到达):
    - 内容: { "AgvId": "AGV001", "StationId": "nlwW-3g8", "RequestId": 123456 }
    - 作用: 触发界面显示“请扫码”，并记录 [SseRequestId] 用于后续确认。
  3. AGV_DEPART (AGV离开):
    - 内容: 同上。
    - 作用: 重置界面状态，准备下一个任务。
  4. WAVE_CLOSE (波次结束):
    - 内容: { "WaveId": "W1001", ... }
    - 作用: 通知当前分拣任务批次结束。
B. HTTP 登录接口

- 调用方法: [WcsClient.Login(LoginInputDto)]
- URL: {WcsServerUrl}station/operation/login
- Method: POST
- Header:
  - accept-language: zh-CN
  - requestId: GUID
- Body (JSON - [LoginInputDto]:
{
  "PlatformId": "5Z7fVd...",
  "StationId": "nlwW-3g8",
  "Username": "admin",
  "Password": "123456"
}
- Response (JSON - BaseResult<LoginOutputDto>):
{
  "Code": 0,
  "Message": "Success",
  "Data": {
    "Authorization": "Bearer eyJhbGciOi..." // JWT Token
  }
}
- 后续动作: 如果登录成功，[WcsClient] 会将 [Authorization] Token 存入 [_header] 字典，后续所有 HTTP 请求都会自动带上这个 Token。

---

2. 作业循环 (Work Cycle)

这是操作员日常工作的核心流程：AGV到达 -> 扫码 -> 获取任务 -> 投递 -> AGV离开。

步骤 1: AGV 到达 (由 SSE 触发)

- 来源: SSE 推送 [Agv_Arrived]消息。
- 内部处理: [SupplyStationViewModel.AGVArrived(...)]
- 关键动作:
  1. 调用 [WcsClient.VerifyNotify]确认收到消息。
  2. 界面提示“请扫码”。
  3. 状态机流转至 [EnumSupplyFlow.AgvArrived]。
步骤 2: 扫码与任务查询 (HTTP)

当扫描枪或相机识别到条码后：

- 调用方法 1: [WcsClient.ScanBarcode(ItemInfoVosInputDto)]
  - URL: {WcsServerUrl}station/operation/scanBarcode
  - Body:
{ "Barcode": "690123456789" }
  - Response: 返回该条码对应的商品简要信息 ([StationItemInfoVo])。
- 调用方法 2: [WcsClient.GetTaskInfo(TaskInfoVosInputDto)]
  - URL: {WcsServerUrl}station/operation/getTaskInfo
  - Body:
{
  "StationId": "nlwW-3g8",
  "Sku": "SKU123", // 可选
  "Barcode": "690123456789",
  "Completed": 0,
  "WaveType": "ORDER", // 或 PICKING
  "LockId": ""
}
  - Response ([TaskInfoVosOutputDto]:
{
  "Code": 0,
  "Data": {
    "TaskId": "T20231027001",
    "ChuteId": "CHUTE-05", // 格口编号
    "WaveId": "W1001",
    "OrderId": "ORD-999",
    "Count": 1 // 需要投递的数量
  }
}
  - 界面反馈: 显示格口灯亮、显示商品信息、显示剩余数量。
步骤 3: 投递确认与 AGV 发车 (HTTP)

当操作员将包裹放入格口并确认后（或自动计数完成）：

- 调用方法: [WcsClient.RobotDeparture(RobotDepartureInputDto)]
  - URL: {WcsServerUrl}station/operation/robotDeparture
  - Header: 包含 requestId (通常使用 SSE 传来的那个 ID 或新生成的)
  - Body:
{
  "TaskId": "T20231027001",
  "AgvId": "AGV001",
  "Completed": 1, // 已完成数量
  "RequestId": "uuid-gen-here"
}
  - Response: 告知 WCS 该 AGV 可以离开。
- 备选方法 (强制发车/无码发车):
  - [WcsClient.DriveOutRobot(...)]: 用于空车驱离。
  - WcsClient.NoBarcodeForceDepart(...): 用于异常情况下强制结束任务。
步骤 4: AGV 离开 (由 SSE 触发)

- 来源: SSE 推送 [AGV_DEPART] 消息。
- 内部处理: SupplyStationViewModel.AGVArrived(..., isArrived: false)
- 动作:
  1. 清空当前任务信息。
  2. 界面重置，等待下一辆 AGV。
  3. 状态机流转回 [EnumSupplyFlow.Inition]。

---

总结：SSE 连接部分的特殊细节

1. 断线重连: JLeap.Common/Network/Sse/SseRequest.cs 中有一个 while 循环。如果连接断开或发生异常，它会等待几秒后自动尝试重新 POST 连接，保证实时性不丢失。
2. 消息确认机制: 注意 [VerifyNotify]接口。当 SSE 收到重要事件（如 AGV 到达）时，客户端必须立即调用这个 HTTP 接口告诉服务器“我收到了”，否则服务器可能会认为客户端离线或重复推送。
3. RequestId 关联: SSE 消息中的 [SseRequestId]非常重要。在后续的 [RobotDeparture]或 [VerifyNotify]中，往往需要带上这个 ID，以便服务器将 HTTP 操作与特定的 SSE 事件上下文关联起来。
数据流向图解

sequenceDiagram
    participant Client as 工作站客户端
    participant HTTP as WCS HTTP API
    participant SSE as WCS SSE Stream
    
    Note over Client, SSE: 1. 启动与连接
    Client->>SSE: POST /connect (Json: StationInfo)
    SSE-->>Client: 保持连接开放 (Text/Event-Stream)
    
    Note over Client, HTTP: 2. 登录
    Client->>HTTP: POST /login (User/Pass)
    HTTP-->>Client: Json: { Authorization: "Token..." }
    
    Note over Client, SSE: 3. 作业循环 - AGV到达
    SSE-->>Client: Event: Agv_Arrived { AgvId, RequestId }
    Client->>HTTP: POST /verifyNotify { SseRequestId }
    
    Note over Client, HTTP: 4. 作业循环 - 扫码
    Client->>HTTP: POST /scanBarcode { Barcode }
    HTTP-->>Client: Json: ItemInfo
    Client->>HTTP: POST /getTaskInfo { Barcode, StationId }
    HTTP-->>Client: Json: { TaskId, ChuteId, Count }
    
    Note over Client, HTTP: 5. 作业循环 - 发车
    Client->>HTTP: POST /robotDeparture { TaskId, AgvId, Completed }
    HTTP-->>Client: Json: Success
    
    Note over Client, SSE: 6. 作业循环 - AGV离开
    SSE-->>Client: Event: AGV_DEPART { AgvId }
    Client->>Client: 重置界面，等待下一次