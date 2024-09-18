package config

type ServiceConfig struct {
	Server     ServerConfig
	DB         PoolConfig
	DataLoader DataLoaderConfig
}

type ServerConfig struct {
	Port           int `env:"CFG__SERVER__PORT,default=8080"`
	RequestTimeout int `env:"CFG__SERVER__REQUEST_TIMEOUT, default=10000"`
}

type PoolConfig struct {
	DbName  string `env:"CFG__DB__DB_NAME, default=postgres"`
	User    string `env:"CFG__DB__USER, default=postgres"`
	Pass    string `env:"CFG__DB__PASS, default=password"`
	Host    string `env:"CFG__DB__HOST, default=127.0.0.1"`
	Port    uint16 `env:"CFG__DB__PORT, default=5432"`
	MaxConn int    `env:"CFG__DB__MAX_CONN, default=16"`
}

type DataLoaderConfig struct {
	DefaultDelayMs int `env:"CFG__DATA_LOADER__DEFAULT_DELAY_MS, default=10"`
	MaxBatchSize   int `env:"CFG__DATA_LOADER__MAX_BATCH_SIZE, default=100"`
}
