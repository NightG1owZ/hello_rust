-- 用户表：注册/登录功能的核心持久化结构
CREATE TABLE IF NOT EXISTS users (
    -- 主键使用 UUID 字符串：便于分布式场景下客户端生成、也避免自增 ID 泄露业务规模
    id            TEXT PRIMARY KEY NOT NULL,
    -- 用户名唯一，登录时可用用户名或邮箱
    username      TEXT NOT NULL UNIQUE,
    -- 邮箱唯一：数据库层面的唯一约束是最后一道防线，业务层先查一次只是为了让报错更友好
    email         TEXT NOT NULL UNIQUE,
    -- 只存哈希，绝不存明文；bcrypt 输出自带算法/代价/盐，长度固定 60 字符
    password_hash TEXT NOT NULL,
    -- 角色：user / admin，用文本存储便于人工排查
    role          TEXT NOT NULL DEFAULT 'user',
    -- RFC3339 字符串存储时间，SQLite 无原生日期类型
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- 登录查询走 email，建索引避免全表扫描
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);
