# DVWA Local Test Setup Guide

Damn Vulnerable Web App (DVWA) is a PHP/MySQL web application that is damn vulnerable. Its main goals are to be an aid for security professionals to test their skills and tools in a legal environment.

## 1. Setup DVWA

The easiest way to run DVWA is via Docker.

### Prerequisites
- Docker installed and running.

### Run DVWA
Run the following command in your terminal:

```bash
docker run --rm -it -p 4280:80 vulnerables/web-dvwa
```

- Access DVWA at: `http://localhost:4280`
- Default credentials: `admin` / `password`
- Click "Create / Reset Database" to initialize.

## 2. Run Dragon-R1 Scan against DVWA

Once DVWA is running, you can use Dragon-R1 to scan it.

### Via API (Python Script)

We have provided a test script: `tests/test_dvwa_scan.py`.

```bash
python tests/test_dvwa_scan.py
```

### Via UI

1. Open Dragon-R1 UI (`http://localhost:3000`).
2. Go to "Start New Task".
3. Enter Target: `http://localhost:4280`.
4. Click "Start".

## 3. Expected Results

Dragon-R1 should detect:
- Open port 80 (Recon Phase).
- SQL Injection vulnerabilities (Vuln Scan Phase).
- XSS vulnerabilities (Vuln Scan Phase).

A report will be generated in `Cerberus/gateway/reports/`.
