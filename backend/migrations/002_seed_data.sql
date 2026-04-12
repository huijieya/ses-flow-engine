-- SES 系统默认种子数据
-- 包含默认节点定义和工作流模板

-- 插入默认应用（系统应用）
INSERT INTO ses_apps (id, name, description, database_url, config, status)
VALUES (
    '00000000-0000-0000-0000-000000000000',
    '系统默认应用',
    'SES系统内置默认应用，包含系统节点定义和模板',
    'postgresql://localhost/ses_system',
    '{}',
    'ACTIVE'
)
ON CONFLICT (id) DO NOTHING;

-- ========================================
-- 设备节点定义 (Device Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('chute_operate', '格口批量操作', '批量操作格口（打开/关闭/禁用/启用）', 'device', 'HUB', 
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteIds":{"type":"array","items":{"type":"string"}},"operation":{"type":"string","enum":["open","close","forbidden","enable"]}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"},"failedChutes":{"type":"array","items":{"type":"string"}}}}'::jsonb,
 '{"type":"object","properties":{"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"},"chuteIds":{"type":"array","title":"格口列表","description":"要操作的格口ID列表","items":{"type":"string"}},"operation":{"type":"string","title":"操作类型","enum":["open","close","forbidden","enable"],"default":"open"}},"required":["platformId","chuteIds","operation"]}'::jsonb,
 '{"platformId":"","chuteIds":[],"operation":"open"}'::jsonb,
 'Box', '#2EC6D6', '设备控制', true),

('chute_open', '打开格口', '打开单个格口', 'device', 'HUB',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"},"chuteId":{"type":"string","title":"格口ID","description":"格口标识"}},"required":["platformId","chuteId"]}'::jsonb,
 '{"platformId":"","chuteId":""}'::jsonb,
 'Open', '#2EC6D6', '设备控制', true),

('chute_close', '关闭格口', '关闭单个格口', 'device', 'HUB',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"},"chuteId":{"type":"string","title":"格口ID","description":"格口标识"}},"required":["platformId","chuteId"]}'::jsonb,
 '{"platformId":"","chuteId":""}'::jsonb,
 'Close', '#F56C6C', '设备控制', true),

('device_command', '设备命令', '向设备发送通用命令', 'device', NULL,
 '{"type":"object","properties":{"deviceId":{"type":"string"},"commandType":{"type":"string"},"data":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"taskId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"deviceId":{"type":"string","title":"设备ID","description":"目标设备标识"},"commandType":{"type":"string","title":"命令类型","description":"命令类型代码"},"data":{"type":"object","title":"命令数据","description":"命令参数对象"}},"required":["deviceId","commandType"]}'::jsonb,
 '{"deviceId":"","commandType":"","data":{}}'::jsonb,
 'Cpu', '#2EC6D6', '设备控制', true),

('printer_print', '打印任务', '发送打印指令', 'device', 'PRINTER',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"},"packId":{"type":"string"},"templateId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"printResult":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"},"chuteId":{"type":"string","title":"格口ID","description":"格口标识"},"packId":{"type":"string","title":"封包ID","description":"封包标识"},"templateId":{"type":"string","title":"模板ID","description":"打印模板标识"}},"required":["platformId","packId"]}'::jsonb,
 '{"platformId":"","chuteId":"","packId":"","templateId":""}'::jsonb,
 'Printer', '#2EC6D6', '设备控制', true),

('device_set_light', '设置格口灯', '设置控制器灯光', 'device', 'HUB',
 '{"type":"object","properties":{"deviceId":{"type":"string"},"nodeId":{"type":"integer"},"lightConfig":{"type":"object","properties":{"red":{"type":"boolean"},"green":{"type":"boolean"},"yellow":{"type":"boolean"}}}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"deviceId":{"type":"string","title":"设备ID","description":"控制器设备标识"},"nodeId":{"type":"integer","title":"节点ID","description":"灯控节点编号"},"lightConfig":{"type":"object","title":"灯光配置","properties":{"red":{"type":"boolean","title":"红灯","default":false},"green":{"type":"boolean","title":"绿灯","default":false},"yellow":{"type":"boolean","title":"黄灯","default":false}}}},"required":["deviceId","nodeId"]}'::jsonb,
 '{"deviceId":"","nodeId":0,"lightConfig":{"red":false,"green":false,"yellow":false}}'::jsonb,
 'Light', '#E6A23C', '设备控制', true),

('device_set_buzzer', '设置蜂鸣器', '设置蜂鸣器模式', 'device', 'HUB',
 '{"type":"object","properties":{"deviceId":{"type":"string"},"controllerId":{"type":"string"},"buzzerMode":{"type":"string","enum":["off","short","long","continuous"]}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"deviceId":{"type":"string","title":"设备ID","description":"控制器设备标识"},"controllerId":{"type":"string","title":"控制器ID","description":"蜂鸣器控制器标识"},"buzzerMode":{"type":"string","title":"蜂鸣模式","enum":["off","short","long","continuous"],"default":"short","description":"蜂鸣器工作模式"}},"required":["deviceId","buzzerMode"]}'::jsonb,
 '{"deviceId":"","controllerId":"","buzzerMode":"short"}'::jsonb,
 'Bell', '#E6A23C', '设备控制', true),

('wall_online', '播种墙上线', '播种墙上线操作', 'device', 'WALL',
 '{"type":"object","properties":{"mcId":{"type":"string"},"chuteId":{"type":"string"},"rfid":{"type":"string"},"wallLocation":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"wallId":{"type":"string"},"wallTypeId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"mcId":{"type":"string","title":"主控ID","description":"播种墙主控制器ID"},"chuteId":{"type":"string","title":"格口ID","description":"播种墙格口标识"},"rfid":{"type":"string","title":"RFID","description":"播种墙RFID标签"},"wallLocation":{"type":"string","title":"位置","description":"播种墙物理位置"}},"required":["mcId","chuteId"]}'::jsonb,
 '{"mcId":"","chuteId":"","rfid":"","wallLocation":""}'::jsonb,
 'Upload', '#2EC6D6', '设备控制', true),

('wall_offline', '播种墙下线', '播种墙下线操作', 'device', 'WALL',
 '{"type":"object","properties":{"wallId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"wallId":{"type":"string","title":"播种墙ID","description":"播种墙标识"}},"required":["wallId"]}'::jsonb,
 '{"wallId":""}'::jsonb,
 'Download', '#909399', '设备控制', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 逻辑节点定义 (Logic Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('condition_router', '条件路由', '根据条件表达式决定流程走向', 'logic', NULL,
 '{"type":"object","properties":{"condition":{"type":"string"},"value":{}}}'::jsonb,
 '{"type":"object","properties":{"nextNodeId":{"type":"string"},"conditionResult":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"condition":{"type":"string","title":"条件表达式","description":"支持JS语法的条件表达式，如: input.value > 10","default":"true"},"trueBranch":{"type":"string","title":"真分支目标","description":"条件为真时的目标节点ID"},"falseBranch":{"type":"string","title":"假分支目标","description":"条件为假时的目标节点ID"}},"required":["condition"]}'::jsonb,
 '{"condition":"true","trueBranch":"","falseBranch":""}'::jsonb,
 'Share', '#E6A23C', '流程控制', true),

('order_type_router', '订单类型路由', '根据订单类型(WALL/CHUTE)分支', 'logic', NULL,
 '{"type":"object","properties":{"orderType":{"type":"string","enum":["WALL","CHUTE"]}}}'::jsonb,
 '{"type":"object","properties":{"nextNodeId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"wallBranch":{"type":"string","title":"播种墙分支","description":"订单类型为WALL时的目标节点"},"chuteBranch":{"type":"string","title":"格口分支","description":"订单类型为CHUTE时的目标节点"}},"required":[]}'::jsonb,
 '{"wallBranch":"","chuteBranch":""}'::jsonb,
 'Switch', '#E6A23C', '流程控制', true),

('foreach', '循环遍历', '对集合中的每个元素执行子流程', 'logic', NULL,
 '{"type":"object","properties":{"collection":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"item":{},"index":{"type":"integer"},"results":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"collectionPath":{"type":"string","title":"集合路径","description":"输入数据中的集合路径，如: input.orders"},"subFlowId":{"type":"string","title":"子流程ID","description":"要执行的子流程标识"}},"required":["collectionPath"]}'::jsonb,
 '{"collectionPath":"","subFlowId":""}'::jsonb,
 'Refresh', '#409EFF', '流程控制', true),

('fork_join', '并发分支', '并发执行多个分支后聚合', 'logic', NULL,
 '{"type":"object","properties":{"branches":{"type":"array","items":{"type":"string"}}}}'::jsonb,
 '{"type":"object","properties":{"results":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"branches":{"type":"array","title":"分支列表","description":"并发执行的分支ID列表","items":{"type":"string"}},"waitAll":{"type":"boolean","title":"等待全部","description":"是否等待所有分支完成","default":true}},"required":["branches"]}'::jsonb,
 '{"branches":[],"waitAll":true}'::jsonb,
 'Connection', '#409EFF', '流程控制', true),

('wait_for_event', '等待事件', '暂停工作流直到收到指定事件', 'logic', NULL,
 '{"type":"object","properties":{"eventName":{"type":"string"},"timeout":{"type":"integer"}}}'::jsonb,
 '{"type":"object","properties":{"eventData":{"type":"object"},"timeout":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"eventName":{"type":"string","title":"事件名称","description":"要等待的事件标识"},"timeoutSeconds":{"type":"integer","title":"超时时间(秒)","description":"等待超时时间，0表示不超时","default":300}},"required":["eventName"]}'::jsonb,
 '{"eventName":"","timeoutSeconds":300}'::jsonb,
 'Timer', '#909399', '流程控制', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 数据节点定义 (Data Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('chute_query', '查询格口', '查询格口信息和状态', 'data', NULL,
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"chuteInfo":{"type":"object","properties":{"chuteId":{"type":"string"},"status":{"type":"string"},"type":{"type":"string"}}}}}'::jsonb,
 '{"type":"object","properties":{"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"},"chuteId":{"type":"string","title":"格口ID","description":"格口标识，支持多个用逗号分隔"}},"required":["platformId"]}'::jsonb,
 '{"platformId":"","chuteId":""}'::jsonb,
 'Search', '#67C23A', '数据查询', true),

('order_query', '查询订单', '查询订单信息', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"orderInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"},"orderId":{"type":"string","title":"订单ID","description":"订单标识"}},"required":[]}'::jsonb,
 '{"waveId":"","orderId":""}'::jsonb,
 'Document', '#67C23A', '数据查询', true),

('wave_query', '查询波次', '查询波次信息', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"}},"required":["waveId"]}'::jsonb,
 '{"waveId":""}'::jsonb,
 'Collection', '#67C23A', '数据查询', true),

('order_create', '创建订单', '创建新订单记录', 'data', NULL,
 '{"type":"object","properties":{"orderInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"orderId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"orderType":{"type":"string","title":"订单类型","enum":["CHUTE","WALL"],"default":"CHUTE"},"priority":{"type":"integer","title":"优先级","default":0,"description":"订单优先级，数值越大优先级越高"},"autoAssign":{"type":"boolean","title":"自动分配","default":true,"description":"是否自动分配格口"}},"required":["orderType"]}'::jsonb,
 '{"orderType":"CHUTE","priority":0,"autoAssign":true}'::jsonb,
 'Plus', '#67C23A', '数据操作', true),

('order_update_status', '更新订单状态', '更新订单状态', 'data', NULL,
 '{"type":"object","properties":{"orderId":{"type":"string"},"status":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"orderId":{"type":"string","title":"订单ID","description":"订单标识"},"newStatus":{"type":"string","title":"新状态","enum":["UN_STARTED","STARTED","IN_PROGRESS","COMPLETED","CLOSED","CANCELLED"],"description":"目标状态"}},"required":["orderId","newStatus"]}'::jsonb,
 '{"orderId":"","newStatus":"STARTED"}'::jsonb,
 'Edit', '#67C23A', '数据操作', true),

('pack_query', '查询封包', '查询封包信息', 'data', NULL,
 '{"type":"object","properties":{"packId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"packInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"packId":{"type":"string","title":"封包ID","description":"封包标识"}},"required":["packId"]}'::jsonb,
 '{"packId":""}'::jsonb,
 'Box', '#67C23A', '数据查询', true),

('parcel_query', '查询包裹', '查询包裹信息', 'data', NULL,
 '{"type":"object","properties":{"parcelId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"parcelInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"parcelId":{"type":"string","title":"包裹ID","description":"包裹标识"}},"required":["parcelId"]}'::jsonb,
 '{"parcelId":""}'::jsonb,
 'Goods', '#67C23A', '数据查询', true),

('item_query', '查询商品', '查询商品信息', 'data', NULL,
 '{"type":"object","properties":{"barcode":{"type":"string"},"sku":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"itemInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"barcode":{"type":"string","title":"条码","description":"商品条码"},"sku":{"type":"string","title":"SKU","description":"商品SKU编码"}},"required":[]}'::jsonb,
 '{"barcode":"","sku":""}'::jsonb,
 'GoodsFilled', '#67C23A', '数据查询', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 波次管理节点定义 (Wave Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('wave_create', '下发波次', '创建新波次', 'data', NULL,
 '{"type":"object","properties":{"waveInfo":{"type":"object"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveName":{"type":"string","title":"波次名称","description":"波次名称"},"priority":{"type":"integer","title":"优先级","default":0,"description":"波次优先级"},"autoStart":{"type":"boolean","title":"自动启动","default":false,"description":"创建后自动启动波次"}},"required":["waveName"]}'::jsonb,
 '{"waveName":"","priority":0,"autoStart":false}'::jsonb,
 'CirclePlus', '#2EC6D6', '波次管理', true),

('wave_start', '启动波次', '启动波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveStatus":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"}},"required":["waveId"]}'::jsonb,
 '{"waveId":""}'::jsonb,
 'VideoPlay', '#2EC6D6', '波次管理', true),

('wave_close', '关闭波次', '关闭波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"},"forceClose":{"type":"boolean","title":"强制关闭","default":false,"description":"是否强制关闭未完成的波次"}},"required":["waveId"]}'::jsonb,
 '{"waveId":"","forceClose":false}'::jsonb,
 'CircleClose', '#909399', '波次管理', true),

('wave_append', '波次追加', '追加订单到波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"},"orders":{"type":"array","title":"订单列表","description":"要追加的订单ID列表","items":{"type":"string"}}},"required":["waveId"]}'::jsonb,
 '{"waveId":"","orders":[]}'::jsonb,
 'Plus', '#2EC6D6', '波次管理', true),

('wave_cancel_orders', '取消波次订单', '取消波次中的订单', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string","title":"波次ID","description":"波次标识"},"orderIds":{"type":"array","title":"订单ID列表","description":"要取消的订单ID列表","items":{"type":"string"}},"reason":{"type":"string","title":"取消原因","description":"取消原因说明"}},"required":["waveId","orderIds"]}'::jsonb,
 '{"waveId":"","orderIds":[],"reason":""}'::jsonb,
 'Remove', '#F56C6C', '波次管理', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 系统节点定义 (System Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('send_email', '发送邮件', '发送邮件通知', 'system', NULL,
 '{"type":"object","properties":{"recipient":{"type":"string"},"subject":{"type":"string"},"body":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"to":{"type":"array","title":"收件人","description":"收件人邮箱地址列表","items":{"type":"string"}},"subject":{"type":"string","title":"邮件主题","description":"邮件主题"},"template":{"type":"string","title":"邮件模板","description":"使用的邮件模板名称"}},"required":["to","subject"]}'::jsonb,
 '{"to":[],"subject":"","template":""}'::jsonb,
 'Message', '#E6A23C', '系统操作', true),

('log_record', '记录日志', '记录系统日志', 'system', NULL,
 '{"type":"object","properties":{"level":{"type":"string","enum":["debug","info","warn","error"]},"message":{"type":"string"},"data":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"level":{"type":"string","title":"日志级别","enum":["debug","info","warn","error"],"default":"info"},"message":{"type":"string","title":"日志内容","description":"日志消息内容"}},"required":["level","message"]}'::jsonb,
 '{"level":"info","message":""}'::jsonb,
 'DocumentCopy', '#909399', '系统操作', true),

('file_upload', '上传文件', '上传文件', 'system', NULL,
 '{"type":"object","properties":{"file":{"type":"string"},"fileType":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"fileUrl":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"sourcePath":{"type":"string","title":"源文件路径","description":"要上传的文件路径"},"targetDir":{"type":"string","title":"目标目录","description":"上传后的存储目录"},"fileType":{"type":"string","title":"文件类型","enum":["image","document","data","other"]}},"required":["sourcePath"]}'::jsonb,
 '{"sourcePath":"","targetDir":"","fileType":"other"}'::jsonb,
 'Upload', '#2EC6D6', '系统操作', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- PDA/工作站节点定义 (PDA/Station Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('pda_scan', 'PDA扫描', 'PDA扫描条码', 'device', 'SCANNER',
 '{"type":"object","properties":{"barcode":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"skuInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"timeoutSeconds":{"type":"integer","title":"超时时间(秒)","default":30,"description":"扫描超时时间"},"requireConfirm":{"type":"boolean","title":"需要确认","default":true,"description":"扫描后是否需要人工确认"}},"required":[]}'::jsonb,
 '{"timeoutSeconds":30,"requireConfirm":true}'::jsonb,
 'FullScreen', '#2EC6D6', 'PDA操作', true),

('pda_distribute', '人工播种', '执行人工播种操作', 'device', 'PDA',
 '{"type":"object","properties":{"taskId":{"type":"string"},"sku":{"type":"string"},"barcode":{"type":"string"},"waveId":{"type":"string"},"orderId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"requireConfirm":{"type":"boolean","title":"需要确认","default":true,"description":"播种后是否需要确认"},"allowOverride":{"type":"boolean","title":"允许覆盖","default":false,"description":"是否允许覆盖已有播种"}},"required":[]}'::jsonb,
 '{"requireConfirm":true,"allowOverride":false}'::jsonb,
 'Pointer', '#2EC6D6', 'PDA操作', true),

('pda_pack', 'PDA封包', 'PDA封包操作', 'device', 'PDA',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"packId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"autoPrint":{"type":"boolean","title":"自动打印","default":true,"description":"封包后自动打印面单"},"printTemplate":{"type":"string","title":"打印模板","description":"面单打印模板"}},"required":[]}'::jsonb,
 '{"autoPrint":true,"printTemplate":""}'::jsonb,
 'Box', '#2EC6D6', 'PDA操作', true),

('station_login', '工作站登录', '工作站登录', 'device', 'STATION',
 '{"type":"object","properties":{"stationId":{"type":"string"},"platformId":{"type":"string"},"username":{"type":"string"},"password":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"token":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"stationId":{"type":"string","title":"工作站ID","description":"工作站标识"},"platformId":{"type":"string","title":"平台ID","description":"设备平台标识"}},"required":["stationId","platformId"]}'::jsonb,
 '{"stationId":"","platformId":""}'::jsonb,
 'User', '#2EC6D6', '工作站操作', true),

('station_get_task', '获取任务', '工作站获取任务', 'data', NULL,
 '{"type":"object","properties":{"stationId":{"type":"string"},"platformId":{"type":"string"},"sku":{"type":"string"},"barcode":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"taskInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"taskType":{"type":"string","title":"任务类型","enum":["SORTING","PACKING","INDUCTION","ALL"],"default":"ALL","description":"要获取的任务类型"},"maxTasks":{"type":"integer","title":"最大任务数","default":1,"description":"最多获取的任务数量"}},"required":[]}'::jsonb,
 '{"taskType":"ALL","maxTasks":1}'::jsonb,
 'List', '#67C23A', '工作站操作', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 添加开始和结束系统节点
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('start', '开始', '工作流开始节点', 'system', NULL,
 '{}'::jsonb,
 '{}'::jsonb,
 '{"type":"object","properties":{"description":{"type":"string","title":"描述","description":"节点描述说明"}}}'::jsonb,
 '{"description":""}'::jsonb,
 'CircleCheck', '#67C23A', '系统节点', true),

('end', '结束', '工作流结束节点', 'system', NULL,
 '{}'::jsonb,
 '{}'::jsonb,
 '{"type":"object","properties":{"description":{"type":"string","title":"描述","description":"节点描述说明"},"returnData":{"type":"boolean","title":"返回数据","default":false,"description":"是否返回执行数据"}}}'::jsonb,
 '{"description":"","returnData":false}'::jsonb,
 'CircleCloseFilled', '#909399', '系统节点', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 默认工作流模板
-- ========================================

-- 1. 正向分拣主工作流模板
INSERT INTO ses_flows (id, app_id, name, description, flow_json, version, is_template, status, created_by)
VALUES (
    '11111111-1111-1111-1111-111111111111',
    '00000000-0000-0000-0000-000000000000',
    '正向分拣主工作流',
    '标准的正向分拣业务处理流程模板，包含波次创建、订单分配、格口操作、任务下发等节点',
    '{
        "nodes": [
            {"id": "start", "node_def_id": "start", "name": "开始", "kind": "system", "position": {"x": 100, "y": 100}, "config": {}},
            {"id": "wave_create", "node_def_id": "wave_create", "name": "创建波次", "kind": "data", "position": {"x": 300, "y": 100}, "config": {"waveName": "测试波次", "priority": 0, "autoStart": false}},
            {"id": "order_type_router", "node_def_id": "order_type_router", "name": "订单类型路由", "kind": "logic", "position": {"x": 500, "y": 100}, "config": {"wallBranch": "", "chuteBranch": ""}},
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询格口", "kind": "data", "position": {"x": 700, "y": 50}, "config": {"platformId": "", "chuteId": ""}},
            {"id": "chute_operate", "node_def_id": "chute_operate", "name": "打开格口", "kind": "device", "position": {"x": 900, "y": 50}, "config": {"platformId": "", "chuteIds": [], "operation": "open"}},
            {"id": "device_command", "node_def_id": "device_command", "name": "下发分拣任务", "kind": "device", "position": {"x": 1100, "y": 50}, "config": {"deviceId": "", "commandType": "SORTING", "data": {}}},
            {"id": "wait_for_callback", "node_def_id": "wait_for_event", "name": "等待任务完成", "kind": "logic", "position": {"x": 1300, "y": 50}, "config": {"eventName": "TASK_COMPLETED", "timeoutSeconds": 300}},
            {"id": "wave_close", "node_def_id": "wave_close", "name": "关闭波次", "kind": "data", "position": {"x": 700, "y": 150}, "config": {"waveId": "", "forceClose": false}},
            {"id": "end", "node_def_id": "end", "name": "结束", "kind": "system", "position": {"x": 1500, "y": 100}, "config": {}}
        ],
        "edges": [
            {"id": "e1", "source": "start", "target": "wave_create", "condition": null},
            {"id": "e2", "source": "wave_create", "target": "order_type_router", "condition": null},
            {"id": "e3", "source": "order_type_router", "target": "chute_query", "condition": "CHUTE"},
            {"id": "e4", "source": "chute_query", "target": "chute_operate", "condition": null},
            {"id": "e5", "source": "chute_operate", "target": "device_command", "condition": null},
            {"id": "e6", "source": "device_command", "target": "wait_for_callback", "condition": null},
            {"id": "e7", "source": "wait_for_callback", "target": "end", "condition": null},
            {"id": "e8", "source": "order_type_router", "target": "wave_close", "condition": "WALL"},
            {"id": "e9", "source": "wave_close", "target": "end", "condition": null}
        ],
        "variables": {}
    }'::jsonb,
    1, true, 'PUBLISHED', 'system'
)
ON CONFLICT (id) DO NOTHING;

-- 2. 格口异常处理工作流模板
INSERT INTO ses_flows (id, app_id, name, description, flow_json, version, is_template, status, created_by)
VALUES (
    '22222222-2222-2222-2222-222222222222',
    '00000000-0000-0000-0000-000000000000',
    '格口异常处理工作流',
    '处理格口满包、故障等异常情况的标准流程',
    '{
        "nodes": [
            {"id": "start", "node_def_id": "start", "name": "开始", "kind": "system", "position": {"x": 100, "y": 100}, "config": {}},
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询格口状态", "kind": "data", "position": {"x": 300, "y": 100}, "config": {"platformId": "", "chuteId": ""}},
            {"id": "condition_check", "node_def_id": "condition_router", "name": "检查是否满包", "kind": "logic", "position": {"x": 500, "y": 100}, "config": {"condition": "chuteInfo.status == FULL", "trueBranch": "", "falseBranch": ""}},
            {"id": "chute_close", "node_def_id": "chute_close", "name": "关闭格口", "kind": "device", "position": {"x": 700, "y": 50}, "config": {"platformId": "", "chuteId": ""}},
            {"id": "set_red_light", "node_def_id": "device_set_light", "name": "亮红灯", "kind": "device", "position": {"x": 900, "y": 50}, "config": {"deviceId": "", "nodeId": 0, "lightConfig": {"red": true, "green": false, "yellow": false}}},
            {"id": "send_email", "node_def_id": "send_email", "name": "发送通知", "kind": "system", "position": {"x": 1100, "y": 50}, "config": {"to": [], "subject": "格口满包通知", "template": ""}},
            {"id": "wait_clear", "node_def_id": "wait_for_event", "name": "等待清包", "kind": "logic", "position": {"x": 1300, "y": 50}, "config": {"eventName": "CHUTE_CLEARED", "timeoutSeconds": 3600}},
            {"id": "chute_open", "node_def_id": "chute_open", "name": "重新打开格口", "kind": "device", "position": {"x": 1500, "y": 50}, "config": {"platformId": "", "chuteId": ""}},
            {"id": "turn_off_light", "node_def_id": "device_set_light", "name": "关闭指示灯", "kind": "device", "position": {"x": 1700, "y": 50}, "config": {"deviceId": "", "nodeId": 0, "lightConfig": {"red": false, "green": false, "yellow": false}}},
            {"id": "end", "node_def_id": "end", "name": "结束", "kind": "system", "position": {"x": 1900, "y": 100}, "config": {}}
        ],
        "edges": [
            {"id": "e1", "source": "start", "target": "chute_query", "condition": null},
            {"id": "e2", "source": "chute_query", "target": "condition_check", "condition": null},
            {"id": "e3", "source": "condition_check", "target": "chute_close", "condition": "true"},
            {"id": "e4", "source": "chute_close", "target": "set_red_light", "condition": null},
            {"id": "e5", "source": "set_red_light", "target": "send_email", "condition": null},
            {"id": "e6", "source": "send_email", "target": "wait_clear", "condition": null},
            {"id": "e7", "source": "wait_clear", "target": "chute_open", "condition": null},
            {"id": "e8", "source": "chute_open", "target": "turn_off_light", "condition": null},
            {"id": "e9", "source": "turn_off_light", "target": "end", "condition": null},
            {"id": "e10", "source": "condition_check", "target": "end", "condition": "false"}
        ],
        "variables": {}
    }'::jsonb,
    1, true, 'PUBLISHED', 'system'
)
ON CONFLICT (id) DO NOTHING;

-- 3. PDA工作站工作流模板
INSERT INTO ses_flows (id, app_id, name, description, flow_json, version, is_template, status, created_by)
VALUES (
    '33333333-3333-3333-3333-333333333333',
    '00000000-0000-0000-0000-000000000000',
    'PDA工作站工作流',
    'PDA工作站扫码、获取任务、执行播种的完整流程',
    '{
        "nodes": [
            {"id": "start", "node_def_id": "start", "name": "开始", "kind": "system", "position": {"x": 100, "y": 100}, "config": {}},
            {"id": "station_login", "node_def_id": "station_login", "name": "工作站登录", "kind": "device", "position": {"x": 300, "y": 100}, "config": {"stationId": "", "platformId": ""}},
            {"id": "pda_scan", "node_def_id": "pda_scan", "name": "扫描条码", "kind": "device", "position": {"x": 500, "y": 100}, "config": {"timeoutSeconds": 30, "requireConfirm": true}},
            {"id": "item_query", "node_def_id": "item_query", "name": "查询商品", "kind": "data", "position": {"x": 700, "y": 100}, "config": {"barcode": "", "sku": ""}},
            {"id": "station_get_task", "node_def_id": "station_get_task", "name": "获取任务", "kind": "data", "position": {"x": 900, "y": 100}, "config": {"taskType": "ALL", "maxTasks": 1}},
            {"id": "condition_router", "node_def_id": "condition_router", "name": "是否有任务", "kind": "logic", "position": {"x": 1100, "y": 100}, "config": {"condition": "taskInfo != null", "trueBranch": "", "falseBranch": ""}},
            {"id": "pda_distribute", "node_def_id": "pda_distribute", "name": "执行播种", "kind": "device", "position": {"x": 1300, "y": 50}, "config": {"requireConfirm": true, "allowOverride": false}},
            {"id": "order_update_status", "node_def_id": "order_update_status", "name": "更新订单状态", "kind": "data", "position": {"x": 1500, "y": 50}, "config": {"orderId": "", "newStatus": "COMPLETED"}},
            {"id": "pda_pack", "node_def_id": "pda_pack", "name": "封包", "kind": "device", "position": {"x": 1700, "y": 50}, "config": {"autoPrint": true, "printTemplate": ""}},
            {"id": "printer_print", "node_def_id": "printer_print", "name": "打印面单", "kind": "device", "position": {"x": 1900, "y": 50}, "config": {"platformId": "", "packId": ""}},
            {"id": "log_no_task", "node_def_id": "log_record", "name": "记录无任务", "kind": "system", "position": {"x": 1300, "y": 150}, "config": {"level": "info", "message": "无可用任务"}},
            {"id": "end", "node_def_id": "end", "name": "结束", "kind": "system", "position": {"x": 2100, "y": 100}, "config": {}}
        ],
        "edges": [
            {"id": "e1", "source": "start", "target": "station_login", "condition": null},
            {"id": "e2", "source": "station_login", "target": "pda_scan", "condition": null},
            {"id": "e3", "source": "pda_scan", "target": "item_query", "condition": null},
            {"id": "e4", "source": "item_query", "target": "station_get_task", "condition": null},
            {"id": "e5", "source": "station_get_task", "target": "condition_router", "condition": null},
            {"id": "e6", "source": "condition_router", "target": "pda_distribute", "condition": "true"},
            {"id": "e7", "source": "pda_distribute", "target": "order_update_status", "condition": null},
            {"id": "e8", "source": "order_update_status", "target": "pda_pack", "condition": null},
            {"id": "e9", "source": "pda_pack", "target": "printer_print", "condition": null},
            {"id": "e10", "source": "printer_print", "target": "end", "condition": null},
            {"id": "e11", "source": "condition_router", "target": "log_no_task", "condition": "false"},
            {"id": "e12", "source": "log_no_task", "target": "end", "condition": null}
        ],
        "variables": {}
    }'::jsonb,
    1, true, 'PUBLISHED', 'system'
)
ON CONFLICT (id) DO NOTHING;

-- 4. 简单订单处理工作流模板
INSERT INTO ses_flows (id, app_id, name, description, flow_json, version, is_template, status, created_by)
VALUES (
    '44444444-4444-4444-4444-444444444444',
    '00000000-0000-0000-0000-000000000000',
    '简单订单处理工作流',
    '最简化的订单处理流程，适合快速上手',
    '{
        "nodes": [
            {"id": "start", "node_def_id": "start", "name": "开始", "kind": "system", "position": {"x": 100, "y": 100}, "config": {}},
            {"id": "order_create", "node_def_id": "order_create", "name": "创建订单", "kind": "data", "position": {"x": 300, "y": 100}, "config": {"orderType": "CHUTE", "priority": 0, "autoAssign": true}},
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询可用格口", "kind": "data", "position": {"x": 500, "y": 100}, "config": {"platformId": "", "chuteId": ""}},
            {"id": "chute_operate", "node_def_id": "chute_operate", "name": "打开格口", "kind": "device", "position": {"x": 700, "y": 100}, "config": {"platformId": "", "chuteIds": [], "operation": "open"}},
            {"id": "log_record", "node_def_id": "log_record", "name": "记录日志", "kind": "system", "position": {"x": 900, "y": 100}, "config": {"level": "info", "message": "订单处理完成"}},
            {"id": "end", "node_def_id": "end", "name": "结束", "kind": "system", "position": {"x": 1100, "y": 100}, "config": {}}
        ],
        "edges": [
            {"id": "e1", "source": "start", "target": "order_create", "condition": null},
            {"id": "e2", "source": "order_create", "target": "chute_query", "condition": null},
            {"id": "e3", "source": "chute_query", "target": "chute_operate", "condition": null},
            {"id": "e4", "source": "chute_operate", "target": "log_record", "condition": null},
            {"id": "e5", "source": "log_record", "target": "end", "condition": null}
        ],
        "variables": {}
    }'::jsonb,
    1, true, 'PUBLISHED', 'system'
)
ON CONFLICT (id) DO NOTHING;
