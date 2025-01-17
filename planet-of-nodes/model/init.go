package model

import (
	"context"
	"sync"

	"github.com/go-redis/redis/v8"
	"gorm.io/gorm"
)

type DBM struct {
	DB          *gorm.DB
	RedisClient *redis.Client
	Lock        sync.Mutex
	ctx         context.Context
}

func ModelManager(db *gorm.DB, redisClient *redis.Client) *DBM {
	ctx := context.Background()

	if _, err := redisClient.Ping(ctx).Result(); err != nil {
		return nil
	}

	return &DBM{
		DB:          db,
		RedisClient: redisClient,
		ctx:         ctx,
	}
}
