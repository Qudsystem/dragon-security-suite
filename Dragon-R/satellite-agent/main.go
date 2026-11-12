package main

import (
	"context"
	"encoding/json"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	// Pseudo NATS imports for skeleton
	// "github.com/nats-io/nats.go"
)

// SatelliteConfig holds environmental connection vars
type SatelliteConfig struct {
	MasterAddress string
	AgentID       string
	Region        string
	SecurityToken string
}

// TelemetryPayload sent back to the Master Gateway
type TelemetryPayload struct {
	AgentID   string    `json:"agent_id"`
	Region    string    `json:"region"`
	CPUUsage  float64   `json:"cpu_usage"`
	MemUsage  float64   `json:"mem_usage"`
	Status    string    `json:"status"`
	Timestamp time.Time `json:"timestamp"`
}

func main() {
	log.Println("🚀 Dragon R1 - Satellite Agent Booting...")

	cfg := loadConfig()

	// Connecting to NATS (Simulated for skeleton)
	log.Printf("🔌 Connecting to Master Box at: %s", cfg.MasterAddress)
	// nc, err := nats.Connect(cfg.MasterAddress, nats.Token(cfg.SecurityToken))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Start Telemetry Goroutine
	go startTelemetry(ctx, cfg) // Pass nc inside when imported

	// Wait for OS signals (Graceful Shutdown)
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	log.Println("🛑 Terminating Satellite Agent Securely.")
}

func loadConfig() SatelliteConfig {
	return SatelliteConfig{
		MasterAddress: getEnv("MASTER_ADDRESS", "nats://127.0.0.1:4222"),
		AgentID:       getEnv("AGENT_ID", "sat-node-001"),
		Region:        getEnv("REGION", "cloud-aws-eu"),
		SecurityToken: getEnv("SECURITY_TOKEN", "default_dragon_token"),
	}
}

func startTelemetry(ctx context.Context, cfg SatelliteConfig) {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			payload := TelemetryPayload{
				AgentID:   cfg.AgentID,
				Region:    cfg.Region,
				CPUUsage:  12.5, // Mocked hardware check
				MemUsage:  34.1,
				Status:    "Healthy",
				Timestamp: time.Now(),
			}
			data, _ := json.Marshal(payload)
			// nc.Publish("dragon.telemetry", data)
			log.Printf("📡 Sent Telemetry: %s", string(data))
		}
	}
}

func getEnv(key, fallback string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return fallback
}
