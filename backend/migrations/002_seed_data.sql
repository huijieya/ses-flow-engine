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
 '{"type":"object","properties":{"operation":{"type":"string","default":"open"}}}'::jsonb,
 '{"operation":"open"}'::jsonb,
 'Box', '#2EC6D6', '设备控制', true),

('chute_open', '打开格口', '打开单个格口', 'device', 'HUB',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Open', '#2EC6D6', '设备控制', true),

('chute_close', '关闭格口', '关闭单个格口', 'device', 'HUB',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Close', '#F56C6C', '设备控制', true),

('device_command', '设备命令', '向设备发送通用命令', 'device', NULL,
 '{"type":"object","properties":{"deviceId":{"type":"string"},"commandType":{"type":"string"},"data":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"taskId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"commandType":{"type":"string"}}}'::jsonb,
 '{"commandType":""}'::jsonb,
 'Cpu', '#2EC6D6', '设备控制', true),

('printer_print', '打印任务', '发送打印指令', 'device', 'PRINTER',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"},"packId":{"type":"string"},"templateId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"printResult":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Printer', '#2EC6D6', '设备控制', true),

('device_set_light', '设置格口灯', '设置控制器灯光', 'device', 'HUB',
 '{"type":"object","properties":{"deviceId":{"type":"string"},"nodeId":{"type":"integer"},"lightConfig":{"type":"object","properties":{"red":{"type":"boolean"},"green":{"type":"boolean"},"yellow":{"type":"boolean"}}}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Light', '#E6A23C', '设备控制', true),

('device_set_buzzer', '设置蜂鸣器', '设置蜂鸣器模式', 'device', 'HUB',
 '{"type":"object","properties":{"deviceId":{"type":"string"},"controllerId":{"type":"string"},"buzzerMode":{"type":"string","enum":["off","short","long","continuous"]}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Bell', '#E6A23C', '设备控制', true),

('wall_online', '播种墙上线', '播种墙上线操作', 'device', 'WALL',
 '{"type":"object","properties":{"mcId":{"type":"string"},"chuteId":{"type":"string"},"rfid":{"type":"string"},"wallLocation":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"wallId":{"type":"string"},"wallTypeId":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Upload', '#2EC6D6', '设备控制', true),

('wall_offline', '播种墙下线', '播种墙下线操作', 'device', 'WALL',
 '{"type":"object","properties":{"wallId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Download', '#909399', '设备控制', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 逻辑节点定义 (Logic Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('condition_router', '条件路由', '根据条件表达式决定流程走向', 'logic', NULL,
 '{"type":"object","properties":{"condition":{"type":"string"},"value":{}}}'::jsonb,
 '{"type":"object","properties":{"nextNodeId":{"type":"string"},"conditionResult":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"condition":{"type":"string","description":"条件表达式如: input.value > 10"},"trueBranch":{"type":"string"},"falseBranch":{"type":"string"}}}'::jsonb,
 '{"condition":"true","trueBranch":"","falseBranch":""}'::jsonb,
 'Share', '#E6A23C', '流程控制', true),

('order_type_router', '订单类型路由', '根据订单类型(WALL/CHUTE)分支', 'logic', NULL,
 '{"type":"object","properties":{"orderType":{"type":"string","enum":["WALL","CHUTE"]}}}'::jsonb,
 '{"type":"object","properties":{"nextNodeId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"wallBranch":{"type":"string"},"chuteBranch":{"type":"string"}}}'::jsonb,
 '{"wallBranch":"","chuteBranch":""}'::jsonb,
 'Switch', '#E6A23C', '流程控制', true),

('foreach', '循环遍历', '对集合中的每个元素执行子流程', 'logic', NULL,
 '{"type":"object","properties":{"collection":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"item":{},"index":{"type":"integer"},"results":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"subFlowId":{"type":"string"}}}'::jsonb,
 '{"subFlowId":""}'::jsonb,
 'Refresh', '#409EFF', '流程控制', true),

('fork_join', '并发分支', '并发执行多个分支后聚合', 'logic', NULL,
 '{"type":"object","properties":{"branches":{"type":"array","items":{"type":"string"}}}}'::jsonb,
 '{"type":"object","properties":{"results":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"branches":{"type":"array","items":{"type":"string"}},"waitAll":{"type":"boolean","default":true}}}'::jsonb,
 '{"branches":[],"waitAll":true}'::jsonb,
 'Connection', '#409EFF', '流程控制', true),

('wait_for_event', '等待事件', '暂停工作流直到收到指定事件', 'logic', NULL,
 '{"type":"object","properties":{"eventName":{"type":"string"},"timeout":{"type":"integer"}}}'::jsonb,
 '{"type":"object","properties":{"eventData":{"type":"object"},"timeout":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"eventName":{"type":"string"},"timeoutSeconds":{"type":"integer","default":300}}}'::jsonb,
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
 '{}'::jsonb, '{}'::jsonb,
 'Search', '#67C23A', '数据查询', true),

('order_query', '查询订单', '查询订单信息', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"orderInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Document', '#67C23A', '数据查询', true),

('wave_query', '查询波次', '查询波次信息', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Collection', '#67C23A', '数据查询', true),

('order_create', '创建订单', '创建新订单记录', 'data', NULL,
 '{"type":"object","properties":{"orderInfo":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"orderId":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Plus', '#67C23A', '数据操作', true),

('order_update_status', '更新订单状态', '更新订单状态', 'data', NULL,
 '{"type":"object","properties":{"orderId":{"type":"string"},"status":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Edit', '#67C23A', '数据操作', true),

('pack_query', '查询封包', '查询封包信息', 'data', NULL,
 '{"type":"object","properties":{"packId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"packInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Box', '#67C23A', '数据查询', true),

('parcel_query', '查询包裹', '查询包裹信息', 'data', NULL,
 '{"type":"object","properties":{"parcelId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"parcelInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Goods', '#67C23A', '数据查询', true),

('item_query', '查询商品', '查询商品信息', 'data', NULL,
 '{"type":"object","properties":{"barcode":{"type":"string"},"sku":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"itemInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'GoodsFilled', '#67C23A', '数据查询', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 波次管理节点定义 (Wave Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('wave_create', '下发波次', '创建新波次', 'data', NULL,
 '{"type":"object","properties":{"waveInfo":{"type":"object"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'CirclePlus', '#2EC6D6', '波次管理', true),

('wave_start', '启动波次', '启动波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"waveStatus":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'VideoPlay', '#2EC6D6', '波次管理', true),

('wave_close', '关闭波次', '关闭波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'CircleClose', '#909399', '波次管理', true),

('wave_append', '波次追加', '追加订单到波次', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Plus', '#2EC6D6', '波次管理', true),

('wave_cancel_orders', '取消波次订单', '取消波次中的订单', 'data', NULL,
 '{"type":"object","properties":{"waveId":{"type":"string"},"orderInfos":{"type":"array"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Remove', '#F56C6C', '波次管理', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- 系统节点定义 (System Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('send_email', '发送邮件', '发送邮件通知', 'system', NULL,
 '{"type":"object","properties":{"recipient":{"type":"string"},"subject":{"type":"string"},"body":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{"type":"object","properties":{"smtpConfig":{"type":"object"}}}'::jsonb,
 '{}'::jsonb,
 'Message', '#E6A23C', '系统操作', true),

('log_record', '记录日志', '记录系统日志', 'system', NULL,
 '{"type":"object","properties":{"level":{"type":"string","enum":["debug","info","warn","error"]},"message":{"type":"string"},"data":{"type":"object"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'DocumentCopy', '#909399', '系统操作', true),

('file_upload', '上传文件', '上传文件', 'system', NULL,
 '{"type":"object","properties":{"file":{"type":"string"},"fileType":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"fileUrl":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Upload', '#2EC6D6', '系统操作', true)
ON CONFLICT (node_id) DO NOTHING;

-- ========================================
-- PDA/工作站节点定义 (PDA/Station Nodes)
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('pda_scan', 'PDA扫描', 'PDA扫描条码', 'device', 'SCANNER',
 '{"type":"object","properties":{"barcode":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"skuInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'FullScreen', '#2EC6D6', 'PDA操作', true),

('pda_distribute', '人工播种', '执行人工播种操作', 'device', 'PDA',
 '{"type":"object","properties":{"taskId":{"type":"string"},"sku":{"type":"string"},"barcode":{"type":"string"},"waveId":{"type":"string"},"orderId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"success":{"type":"boolean"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Pointer', '#2EC6D6', 'PDA操作', true),

('pda_pack', 'PDA封包', 'PDA封包操作', 'device', 'PDA',
 '{"type":"object","properties":{"platformId":{"type":"string"},"chuteId":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"packId":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'Box', '#2EC6D6', 'PDA操作', true),

('station_login', '工作站登录', '工作站登录', 'device', 'STATION',
 '{"type":"object","properties":{"stationId":{"type":"string"},"platformId":{"type":"string"},"username":{"type":"string"},"password":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"token":{"type":"string"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'User', '#2EC6D6', '工作站操作', true),

('station_get_task', '获取任务', '工作站获取任务', 'data', NULL,
 '{"type":"object","properties":{"stationId":{"type":"string"},"platformId":{"type":"string"},"sku":{"type":"string"},"barcode":{"type":"string"}}}'::jsonb,
 '{"type":"object","properties":{"taskInfo":{"type":"object"}}}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'List', '#67C23A', '工作站操作', true)
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
            {"id": "wave_create", "node_def_id": "wave_create", "name": "创建波次", "kind": "data", "position": {"x": 300, "y": 100}, "config": {}},
            {"id": "order_type_router", "node_def_id": "order_type_router", "name": "订单类型路由", "kind": "logic", "position": {"x": 500, "y": 100}, "config": {}},
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询格口", "kind": "data", "position": {"x": 700, "y": 50}, "config": {}},
            {"id": "chute_operate", "node_def_id": "chute_operate", "name": "打开格口", "kind": "device", "position": {"x": 900, "y": 50}, "config": {"operation": "open"}},
            {"id": "device_command", "node_def_id": "device_command", "name": "下发分拣任务", "kind": "device", "position": {"x": 1100, "y": 50}, "config": {"commandType": "SORTING"}},
            {"id": "wait_for_callback", "node_def_id": "wait_for_event", "name": "等待任务完成", "kind": "logic", "position": {"x": 1300, "y": 50}, "config": {"eventName": "TASK_COMPLETED"}},
            {"id": "wave_close", "node_def_id": "wave_close", "name": "关闭波次", "kind": "data", "position": {"x": 700, "y": 150}, "config": {}},
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
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询格口状态", "kind": "data", "position": {"x": 300, "y": 100}, "config": {}},
            {"id": "condition_check", "node_def_id": "condition_router", "name": "检查是否满包", "kind": "logic", "position": {"x": 500, "y": 100}, "config": {"condition": "chuteInfo.status == FULL"}},
            {"id": "chute_close", "node_def_id": "chute_close", "name": "关闭格口", "kind": "device", "position": {"x": 700, "y": 50}, "config": {}},
            {"id": "set_red_light", "node_def_id": "device_set_light", "name": "亮红灯", "kind": "device", "position": {"x": 900, "y": 50}, "config": {"lightConfig": {"red": true, "green": false, "yellow": false}}},
            {"id": "send_email", "node_def_id": "send_email", "name": "发送通知", "kind": "system", "position": {"x": 1100, "y": 50}, "config": {}},
            {"id": "wait_clear", "node_def_id": "wait_for_event", "name": "等待清包", "kind": "logic", "position": {"x": 1300, "y": 50}, "config": {"eventName": "CHUTE_CLEARED", "timeoutSeconds": 3600}},
            {"id": "chute_open", "node_def_id": "chute_open", "name": "重新打开格口", "kind": "device", "position": {"x": 1500, "y": 50}, "config": {}},
            {"id": "turn_off_light", "node_def_id": "device_set_light", "name": "关闭指示灯", "kind": "device", "position": {"x": 1700, "y": 50}, "config": {"lightConfig": {"red": false, "green": false, "yellow": false}}},
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
            {"id": "station_login", "node_def_id": "station_login", "name": "工作站登录", "kind": "device", "position": {"x": 300, "y": 100}, "config": {}},
            {"id": "pda_scan", "node_def_id": "pda_scan", "name": "扫描条码", "kind": "device", "position": {"x": 500, "y": 100}, "config": {}},
            {"id": "item_query", "node_def_id": "item_query", "name": "查询商品", "kind": "data", "position": {"x": 700, "y": 100}, "config": {}},
            {"id": "station_get_task", "node_def_id": "station_get_task", "name": "获取任务", "kind": "data", "position": {"x": 900, "y": 100}, "config": {}},
            {"id": "condition_router", "node_def_id": "condition_router", "name": "是否有任务", "kind": "logic", "position": {"x": 1100, "y": 100}, "config": {"condition": "taskInfo != null"}},
            {"id": "pda_distribute", "node_def_id": "pda_distribute", "name": "执行播种", "kind": "device", "position": {"x": 1300, "y": 50}, "config": {}},
            {"id": "order_update_status", "node_def_id": "order_update_status", "name": "更新订单状态", "kind": "data", "position": {"x": 1500, "y": 50}, "config": {}},
            {"id": "pda_pack", "node_def_id": "pda_pack", "name": "封包", "kind": "device", "position": {"x": 1700, "y": 50}, "config": {}},
            {"id": "printer_print", "node_def_id": "printer_print", "name": "打印面单", "kind": "device", "position": {"x": 1900, "y": 50}, "config": {}},
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
            {"id": "order_create", "node_def_id": "order_create", "name": "创建订单", "kind": "data", "position": {"x": 300, "y": 100}, "config": {}},
            {"id": "chute_query", "node_def_id": "chute_query", "name": "查询可用格口", "kind": "data", "position": {"x": 500, "y": 100}, "config": {}},
            {"id": "chute_operate", "node_def_id": "chute_operate", "name": "打开格口", "kind": "device", "position": {"x": 700, "y": 100}, "config": {"operation": "open"}},
            {"id": "log_record", "node_def_id": "log_record", "name": "记录日志", "kind": "system", "position": {"x": 900, "y": 100}, "config": {"level": "info"}},
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

-- ========================================
-- 添加开始和结束系统节点
-- ========================================
INSERT INTO ses_node_definitions (node_id, name, description, kind, device_type, input_schema, output_schema, config_schema, default_config, icon, color, category, is_system) VALUES
('start', '开始', '工作流开始节点', 'system', NULL,
 '{}'::jsonb,
 '{}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'CircleCheck', '#67C23A', '系统节点', true),

('end', '结束', '工作流结束节点', 'system', NULL,
 '{}'::jsonb,
 '{}'::jsonb,
 '{}'::jsonb, '{}'::jsonb,
 'CircleCloseFilled', '#909399', '系统节点', true)
ON CONFLICT (node_id) DO NOTHING;
