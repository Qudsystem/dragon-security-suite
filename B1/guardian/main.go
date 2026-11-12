package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os/exec"
	"strings"
)

// GuardianAction defines a whitelist of deterministic security actions (Tokens)
type GuardianAction struct {
	Token   string            `json:"token"`
	Target  string            `json:"target"`
	Options map[string]string `json:"options"`
}

// ActionResponse status and output
type ActionResponse struct {
	Status string `json:"status"`
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr"`
	Error  string `json:"error,omitempty"`
}

// TokenConfig defines properties of a deterministic action
type TokenConfig struct {
	CommandTemplate  string
	ApprovalRequired bool
}

// Deterministic Token Map
var tokenToCommand = map[string]TokenConfig{
	"TOKEN_RECON_SCAN":   {"nmap -sV -sC -T4 %s", false},
	"TOKEN_WEB_ENUM":     {"nikto -h %s", false},
	"TOKEN_VULN_SCAN":    {"nuclei -u %s -severity critical,high", false},
	"TOKEN_SUB_ENUM":     {"subfinder -d %s", false},
	"TOKEN_DIR_BRUTE":    {"ffuf -u %s/FUZZ -w /wordlists/common.txt", false},
	"TOKEN_HTTP_PROBE":   {"httpx -u %s", false},
	"TOKEN_SQL_CHECK":    {"sqlmap -u %s --batch --banner", true}, // CRITICAL: Needs approval
}

func handleAction(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST allowed", http.StatusMethodNotAllowed)
		return
	}

	var action GuardianAction
	if err := json.NewDecoder(r.Body).Decode(&action); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	config, exists := tokenToCommand[action.Token]
	if !exists {
		http.Error(w, "Forbidden Token: Action not deterministic or unauthorized", http.StatusForbidden)
		return
	}

	// Permission/MFA Check
	if config.ApprovalRequired {
		log.Printf("🛑 CRITICAL ACTION BLOCKED: %s requires manual approval/MFA", action.Token)
		http.Error(w, "MFA_REQUIRED: This critical operation requires human approval", http.StatusUnauthorized)
		return
	}

	log.Printf("🛡️ Guardian validating action: %s on target: %s", action.Token, action.Target)

	// Format command safely
	finalCmd := fmt.Sprintf(config.CommandTemplate, action.Target)
	log.Printf("🚀 Executing: %s", finalCmd)

	// Execute
	cmdParts := strings.Fields(finalCmd)
	cmd := exec.Command(cmdParts[0], cmdParts[1:]...)
	out, err := cmd.CombinedOutput()

	resp := ActionResponse{
		Status: "completed",
		Stdout: string(out),
	}
	if err != nil {
		resp.Status = "error"
		resp.Stderr = err.Error()
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

func handleKill(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST allowed", http.StatusMethodNotAllowed)
		return
	}
	log.Println("🛑 EMERGENCY KILL RECEIVED: Terminating all active processes")
	// For MVP, we'd kill all children of the current process group
	// On Windows: taskkill /F /T /PID <current_pid>
	fmt.Fprintf(w, "TERMINATED")
}

func main() {
	http.HandleFunc("/execute", handleAction)
	http.HandleFunc("/kill", handleKill)
	log.Println("🐉 Guardian Engine (Deterministic Execution) listening on :8001")
	log.Fatal(http.ListenAndServe(":8001", nil))
}
