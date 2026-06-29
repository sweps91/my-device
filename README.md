
# my-device
🦀 Get information about device os, cpu, ram, disks, networks & consuming processes 🚀

### Usage
Clone & run - it is simple to use

```shell
git clone git@github.com:sweps91/my-device.git  # clone repository
cd my-device                                    # go into folder
cargo run                                       # run rust code
```

### Result printed
After `cargo run` command you will see program printed output like this:

```text
MY DEVICE: my-device-name
day: 2026-06-29
time: 14h-25m-53s
timezone: UTC +02:00

SYSTEM:
name:              Windows (arch: x86_64)
os version:        11 (26200)
os long version:   Windows 11 Pro
kernel version:    26200
uptime (hours):    5

CPU:
brand:          12th Gen Intel(R) Core(TM) i5-1235U
vendor_id:      GenuineIntel
number of CPUs: 12
cpu usage: 6.9% (below per unit)
cpu 1: 20.3%
cpu 2: 0.6%
cpu 3: 0.6%
cpu 4: 25.6%
cpu 5: 9.9%
cpu 6: 5.1%
cpu 7: 4.2%
cpu 8: 13.9%
cpu 9: 1.2%
cpu 10: 0.7%
cpu 11: 0.6%
cpu 12: 0.6%

RAM:
total memory: 15.63 GB
used memory:  11.38 GB (73 %)
total swap:   1.00 GB
used swap:    0.00 GB

DISKS:
SSD: "OS" - total: 475.12 GB, used: 292.96 GB (62 %)
	  - removable: false, file sys: "NTFS", on: "C:\\"

NETWORK:
Ethernet 2: downloading: 0.427 KB, uploading: 0.292 KB

PROCESSES:
Number of processes: 263

TOP 25 RAM PROCESSES:
"rust-analyzer.exe": 990 MB
"Memory Compression": 645 MB
"chrome.exe": 602 MB
"chrome.exe": 530 MB
"chrome.exe": 462 MB
"bdservicehost.exe": 461 MB
"chrome.exe": 431 MB
"chrome.exe": 381 MB
"Code.exe": 378 MB
"explorer.exe": 358 MB
"chrome.exe": 345 MB
"chrome.exe": 259 MB
"chrome.exe": 227 MB
"Code.exe": 210 MB
"Code.exe": 198 MB
"Code.exe": 191 MB
"chrome.exe": 169 MB
"OneDrive.exe": 161 MB
"Code.exe": 148 MB
"StartMenuExperienceHost.exe": 142 MB
"Code.exe": 136 MB
"Code.exe": 130 MB
"SearchHost.exe": 118 MB
"Code.exe": 117 MB
"Code.exe": 113 MB

TOP 15 CPU PROCESSES:
"ASUSDetectDVD.exe": 9.9%
"System": 9.4%
"chrome.exe": 6.4%
"chrome.exe": 3.8%
"chrome.exe": 3.8%
"bdservicehost.exe": 3.7%
"Code.exe": 2.1%
"chrome.exe": 2.0%
"svchost.exe": 1.4%
"explorer.exe": 1.4%
"chrome.exe": 1.3%
"chrome.exe": 1.3%
"svchost.exe": 1.1%
"OneDrive.Sync.Service.exe": 0.9%
"svchost.exe": 0.9%

COMPONENTS:
(need permission: run as administrator)
Computer temperature: 46.850006°C (max: 46.850006°C / critical: 98.850006°C)
```

Note: for components temperature can be required to run program as administrator (Windows) 

### Result saved
- Reports Folder: A folder named `my-device-report` is created in your current working directory.
- File Generation: A new `.txt` file is generated inside this folder for each individual run.

🦀🦀🦀
