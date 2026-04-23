-- Order-Chute binding table for destination assignment
-- 订单与格口绑定关系表

CREATE TABLE IF NOT EXISTS ses_order_chute_bindings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL,
    order_id VARCHAR(64) NOT NULL,
    chute_id VARCHAR(64) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, RELEASED
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    released_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT fk_order FOREIGN KEY (app_id,order_id) REFERENCES ses_orders(app_id,order_id) ON DELETE CASCADE
);

-- Index for quick lookup
CREATE INDEX idx_order_chute_bindings_app_id ON ses_order_chute_bindings(app_id);
CREATE INDEX idx_order_chute_bindings_order_id ON ses_order_chute_bindings(order_id);
CREATE INDEX idx_order_chute_bindings_chute_id ON ses_order_chute_bindings(chute_id);
CREATE INDEX idx_order_chute_bindings_status ON ses_order_chute_bindings(status);

-- Chute grids table - stores chute info from platform
-- 格口网格表 - 存储平台格口信息

CREATE TABLE IF NOT EXISTS ses_chute_grids (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chute_id VARCHAR(64) NOT NULL UNIQUE,
    platform_id VARCHAR(64) NOT NULL,
    grid_name VARCHAR(128),
    grid_type VARCHAR(20) NOT NULL, -- CHUTE, STATION
    port_code VARCHAR(64),
    x_coord INTEGER,
    y_coord INTEGER,
    status VARCHAR(20) DEFAULT 'OPEN', -- OPEN, CLOSE, FORBIDDEN
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_chute_grids_platform_id ON ses_chute_grids(platform_id);
CREATE INDEX idx_chute_grids_status ON ses_chute_grids(status);
CREATE INDEX idx_chute_grids_type ON ses_chute_grids(grid_type);

-- Station table - stores station info
-- 工作站表

CREATE TABLE IF NOT EXISTS ses_stations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    station_id VARCHAR(64) NOT NULL UNIQUE,
    platform_id VARCHAR(64) NOT NULL,
    station_name VARCHAR(128),
    chute_id VARCHAR(64), -- if station has an associated chute
    status VARCHAR(20) DEFAULT 'ONLINE', -- ONLINE, OFFLINE, CLOSE
    wave_id VARCHAR(64),
    current_agv_id VARCHAR(64),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_stations_platform_id ON ses_stations(platform_id);
CREATE INDEX idx_stations_status ON ses_stations(status);
CREATE INDEX idx_stations_wave_id ON ses_stations(wave_id);
