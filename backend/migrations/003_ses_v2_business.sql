-- Add task node kind
ALTER TYPE node_kind ADD VALUE IF NOT EXISTS 'task';

-- Wave-Stations mapping table (which stations are assigned to which wave)
CREATE TABLE IF NOT EXISTS ses_wave_stations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wave_id VARCHAR(100) NOT NULL,
    station_id VARCHAR(100) NOT NULL,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(wave_id, station_id)
);

-- Add wave_id to stations for current wave reference
ALTER TABLE ses_stations ADD COLUMN IF NOT EXISTS wave_id VARCHAR(100);

-- Add index for wave_stations
CREATE INDEX IF NOT EXISTS idx_wave_stations_wave_id ON ses_wave_stations(wave_id);
CREATE INDEX IF NOT EXISTS idx_wave_stations_station_id ON ses_wave_stations(station_id);

-- Seed some demo data for dashboard
INSERT INTO ses_waves (id, wave_id, app_id, wave_name, priority, status, total_orders, completed_orders)
VALUES
    (gen_random_uuid(), 'WV-DEMO-001', '00000000-0000-0000-0000-000000000000', '正向分拣-上午', 1, 'STARTED', 50, 30),
    (gen_random_uuid(), 'WV-DEMO-002', '00000000-0000-0000-0000-000000000000', '正向分拣-下午', 2, 'CREATED', 40, 0),
    (gen_random_uuid(), 'WV-DEMO-003', '00000000-0000-0000-0000-000000000000', '调拨分拣', 1, 'CLOSED', 60, 60)
ON CONFLICT DO NOTHING;

-- Seed some demo stations
INSERT INTO ses_stations (id, station_id, app_id, platform_id, station_name, station_type, status, wave_id)
VALUES
    (gen_random_uuid(), 'ST-001', '00000000-0000-0000-0000-000000000000', 'PLAT-01', '播种工作站A', 'SORTING', 'ONLINE', 'WV-DEMO-001'),
    (gen_random_uuid(), 'ST-002', '00000000-0000-0000-0000-000000000000', 'PLAT-01', '播种工作站B', 'SORTING', 'ONLINE', 'WV-DEMO-001'),
    (gen_random_uuid(), 'ST-003', '00000000-0000-0000-0000-000000000000', 'PLAT-01', '播种工作站C', 'SORTING', 'OFFLINE', NULL),
    (gen_random_uuid(), 'ST-004', '00000000-0000-0000-0000-000000000000', 'PLAT-02', '供货工作站A', 'INDUCTION', 'ONLINE', 'WV-DEMO-001')
ON CONFLICT DO NOTHING;

-- Seed wave-station mappings
INSERT INTO ses_wave_stations (wave_id, station_id)
VALUES
    ('WV-DEMO-001', 'ST-001'),
    ('WV-DEMO-001', 'ST-002'),
    ('WV-DEMO-001', 'ST-004')
ON CONFLICT DO NOTHING;
