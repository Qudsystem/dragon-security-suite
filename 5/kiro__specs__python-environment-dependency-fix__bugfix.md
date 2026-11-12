# Bugfix Requirements Document

## Introduction

The Dragon-R1 gateway fails to start on Windows due to Python dependency installation issues. When users attempt to run the gateway using `python -m uvicorn main:app` or the `start.ps1` script, it fails with "No module named uvicorn" and "No module named 'fastapi'" errors. The root cause is a mismatch between where packages are installed (user site-packages) and where Python looks for them (virtual environment site-packages). Additionally, the `mem0` dependency specified in requirements.txt is not available in PyPI, causing installation failures.

This bug prevents the gateway from starting, blocking all functionality of the Dragon-R1 platform. The issue affects Windows 10 users with Python 3.14 and occurs when the virtual environment is not properly activated before installing dependencies.

## Bug Analysis

### Current Behavior (Defect)

1.1 WHEN a user runs `pip install uvicorn fastapi` without activating the virtual environment THEN the system installs packages to user site-packages (C:\Users\[username]\AppData\Roaming\Python\Python314\site-packages) instead of the virtual environment

1.2 WHEN a user runs `pip install -r requirements.txt` without activating the virtual environment THEN the system installs packages to user site-packages and fails on the `mem0` dependency with "ERROR: Could not find a version that satisfies the requirement mem0"

1.3 WHEN a user runs `python -m uvicorn main:app --host 0.0.0.0 --port 8000` THEN the system uses the virtual environment Python interpreter (C:\Users\[username]\Desktop\RED\.venv\Scripts\python.exe) which cannot find the packages installed in user site-packages

1.4 WHEN the `start.ps1` script executes `.\.venv\Scripts\python.exe -X utf8 main.py` THEN the system fails with "ModuleNotFoundError: No module named 'fastapi'" because the venv Python cannot access user site-packages

1.5 WHEN `pip install -r requirements.txt` is executed THEN the system fails to install the `mem0` package because it is not available as a standalone package in PyPI

### Expected Behavior (Correct)

2.1 WHEN a user runs the gateway setup process THEN the system SHALL ensure the virtual environment is activated before installing any dependencies

2.2 WHEN a user runs `pip install -r requirements.txt` with the virtual environment activated THEN the system SHALL install all packages to the virtual environment site-packages (C:\Users\[username]\Desktop\RED\.venv\Lib\site-packages)

2.3 WHEN a user runs `python -m uvicorn main:app --host 0.0.0.0 --port 8000` with the virtual environment activated THEN the system SHALL successfully start the gateway using packages from the virtual environment

2.4 WHEN the `start.ps1` script executes THEN the system SHALL verify that required dependencies are installed in the virtual environment before attempting to start the gateway

2.5 WHEN `pip install -r requirements.txt` is executed THEN the system SHALL either install the correct `mem0ai` package or gracefully handle the missing `mem0` dependency with a clear error message

2.6 WHEN a user follows the setup instructions THEN the system SHALL provide clear guidance on activating the virtual environment and installing dependencies in the correct location

### Unchanged Behavior (Regression Prevention)

3.1 WHEN the gateway is started with properly installed dependencies THEN the system SHALL CONTINUE TO start FastAPI on port 8000 with all endpoints available

3.2 WHEN the `start.ps1` script checks for Ollama availability THEN the system SHALL CONTINUE TO verify and start Ollama if needed

3.3 WHEN the gateway imports optional modules with try/except fallback THEN the system SHALL CONTINUE TO gracefully degrade when optional dependencies are missing

3.4 WHEN the gateway is run via `python main.py` directly THEN the system SHALL CONTINUE TO start using uvicorn programmatically

3.5 WHEN the `START-DRAGON-R1.bat` script is executed THEN the system SHALL CONTINUE TO start the gateway, databases, and frontend in the correct order

3.6 WHEN dependencies are installed in the virtual environment THEN the system SHALL CONTINUE TO isolate packages from the global Python installation
