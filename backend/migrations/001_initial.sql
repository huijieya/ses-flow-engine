-- Create custom types
CREATE TYPE execution_status AS ENUM ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED', 'RETRY', 'CANCELLED', 'PAUSED');
CREATE TYPE node_kind AS ENUM ('device', 'logic', 'data', 'query', 'system');
CREATE TYPE app_status AS ENUM ('ACTIVE', 'INACTIVE', 'SUSPENDED');
CREATE TYPE flow_status AS ENUM ('DRAFT', 'PUBLISHED', 'ARCHIVED');
CREATE TYPE device_type AS ENUM ('HUB', 'BUTTON', 'LINE', 'LIFT', 'SORTER', 'AGV', 'PRINTER', 'SCANNER');
CREATE TYPE device_status AS ENUM ('IDLE', 'BUSY', 'ERROR', 'OFFLINE', 'MAINTENANCE');
CREATE TYPE chute_type AS ENUM ('NORMAL', 'ERROR');
CREATE TYPE chute_status AS ENUM ('OPEN', 'CLOSE', 'FULL', 'DISABLE', 'ERROR');
CREATE TYPE task_status AS ENUM ('PENDING', 'DISPATCHED', 'RUNNING', 'COMPLETED', 'FAILED', 'CANCELLED', 'TIMEOUT');
CREATE TYPE order_type AS ENUM ('CHUTE', 'WALL');
CREATE TYPE order_status AS ENUM ('UN_STARTED', 'STARTED', 'IN_PROGRESS', 'COMPLETED', 'CLOSED', 'AUTO_COMPLETE', 'CANCELLED');
CREATE TYPE station_type AS ENUM ('INDUCTION', 'PACKING', 'SORTING', 'RETURNS');
CREATE TYPE station_status AS ENUM ('OFFLINE', 'ONLINE', 'BUSY', 'PAUSED');
CREATE TYPE wave_status AS ENUM ('CREATED', 'STARTED', 'PAUSED', 'CLOSED', 'CANCELLED');

-- Apps table
CREATE TABLE ses_apps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    database_url TEXT NOT NULL,
    config JSONB,
    status app_status NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Flow definitions table
CREATE TABLE ses_flows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    flow_json JSONB NOT NULL DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    is_template BOOLEAN NOT NULL DEFAULT FALSE,
    status flow_status NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100)
);

-- Flow instances table
CREATE TABLE ses_flow_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_id UUID NOT NULL REFERENCES ses_flows(id) ON DELETE CASCADE,
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    status execution_status NOT NULL DEFAULT 'PENDING',
    context JSONB,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Node definitions table
CREATE TABLE ses_node_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    kind node_kind NOT NULL,
    device_type VARCHAR(50),
    input_schema JSONB,
    output_schema JSONB,
    config_schema JSONB,
    default_config JSONB,
    icon VARCHAR(100),
    color VARCHAR(50),
    category VARCHAR(100) NOT NULL DEFAULT 'general',
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Node instances table
CREATE TABLE ses_node_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_instance_id UUID NOT NULL REFERENCES ses_flow_instances(id) ON DELETE CASCADE,
    node_def_id VARCHAR(100) NOT NULL,
    node_id VARCHAR(100) NOT NULL,
    node_name VARCHAR(200) NOT NULL,
    node_type VARCHAR(50) NOT NULL,
    input_data JSONB,
    output_data JSONB,
    status execution_status NOT NULL DEFAULT 'PENDING',
    retry_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Devices table
CREATE TABLE ses_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    device_id VARCHAR(100) NOT NULL,
    device_name VARCHAR(200) NOT NULL,
    device_type device_type NOT NULL,
    ip VARCHAR(50),
    port INTEGER,
    online BOOLEAN NOT NULL DEFAULT FALSE,
    status device_status NOT NULL DEFAULT 'OFFLINE',
    config JSONB,
    last_heartbeat TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(app_id, device_id)
);

-- Chutes table
CREATE TABLE ses_chutes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    chute_id VARCHAR(100) NOT NULL,
    platform_id VARCHAR(100) NOT NULL,
    chute_type chute_type NOT NULL DEFAULT 'NORMAL',
    status chute_status NOT NULL DEFAULT 'CLOSE',
    physical_status JSONB,
    io_status JSONB,
    light_status JSONB,
    device_id VARCHAR(100),
    node_id INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(app_id, chute_id)
);

-- Tasks table
CREATE TABLE ses_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    flow_instance_id UUID REFERENCES ses_flow_instances(id) ON DELETE SET NULL,
    node_instance_id UUID REFERENCES ses_node_instances(id) ON DELETE SET NULL,
    device_id VARCHAR(100) NOT NULL,
    task_type VARCHAR(100) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    payload JSONB NOT NULL DEFAULT '{}',
    status task_status NOT NULL DEFAULT 'PENDING',
    result JSONB,
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Waves table
CREATE TABLE ses_waves (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wave_id VARCHAR(100) NOT NULL,
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    wave_name VARCHAR(200) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    status wave_status NOT NULL DEFAULT 'CREATED',
    platform_id VARCHAR(100),
    total_orders INTEGER NOT NULL DEFAULT 0,
    completed_orders INTEGER NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(app_id, wave_id)
);

-- Orders table
CREATE TABLE ses_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id VARCHAR(100) NOT NULL,
    app_id UUID NOT NULL REFERENCES ses_apps(id) ON DELETE CASCADE,
    wave_id VARCHAR(100) NOT NULL,
    order_type order_type NOT NULL,
    chute_id VARCHAR(100),
    status order_status NOT NULL DEFAULT 'UN_STARTED',
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(app_id, order_id)
);

-- Order details table
CREATE TABLE ses_order_details (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id VARCHAR(100) NOT NULL,
    sku VARCHAR(100) NOT NULL,
    barcode VARCHAR(100) NOT NULL,
    qty INTEGER NOT NULL DEFAULT 0,
    completed_qty INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Create indexes
CREATE INDEX idx_flows_app_id ON ses_flows(app_id);
CREATE INDEX idx_flow_instances_flow_id ON ses_flow_instances(flow_id);
CREATE INDEX idx_flow_instances_status ON ses_flow_instances(status);
CREATE INDEX idx_node_instances_flow_instance_id ON ses_node_instances(flow_instance_id);
CREATE INDEX idx_devices_app_id ON ses_devices(app_id);
CREATE INDEX idx_devices_online ON ses_devices(online);
CREATE INDEX idx_chutes_app_id ON ses_chutes(app_id);
CREATE INDEX idx_chutes_platform_id ON ses_chutes(platform_id);
CREATE INDEX idx_tasks_app_id ON ses_tasks(app_id);
CREATE INDEX idx_tasks_device_id ON ses_tasks(device_id);
CREATE INDEX idx_tasks_status ON ses_tasks(status);
CREATE INDEX idx_waves_app_id ON ses_waves(app_id);
CREATE INDEX idx_waves_status ON ses_waves(status);
CREATE INDEX idx_orders_app_id ON ses_orders(app_id);
CREATE INDEX idx_orders_wave_id ON ses_orders(wave_id);
CREATE INDEX idx_orders_status ON ses_orders(status);
CREATE INDEX idx_order_details_order_id ON ses_order_details(order_id);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_ses_apps_updated_at BEFORE UPDATE ON ses_apps FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_flows_updated_at BEFORE UPDATE ON ses_flows FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_flow_instances_updated_at BEFORE UPDATE ON ses_flow_instances FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_node_definitions_updated_at BEFORE UPDATE ON ses_node_definitions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_node_instances_updated_at BEFORE UPDATE ON ses_node_instances FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_devices_updated_at BEFORE UPDATE ON ses_devices FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_chutes_updated_at BEFORE UPDATE ON ses_chutes FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_tasks_updated_at BEFORE UPDATE ON ses_tasks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_waves_updated_at BEFORE UPDATE ON ses_waves FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_orders_updated_at BEFORE UPDATE ON ses_orders FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ses_order_details_updated_at BEFORE UPDATE ON ses_order_details FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
