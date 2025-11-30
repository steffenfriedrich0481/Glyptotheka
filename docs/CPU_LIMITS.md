# CPU and Resource Limits Configuration

This document explains how to configure CPU and memory limits for Glyptotheka on your Unraid NAS.

## Current Configuration

The `docker-compose.yml` includes the following resource limits:

### Backend Container
- **CPU Limit**: 2.0 cores (can use up to 200% of a single core)
- **CPU Reservation**: 0.5 cores minimum
- **Memory Limit**: 2GB maximum
- **Memory Reservation**: 512MB minimum

### Frontend Container
- **CPU Limit**: 0.5 cores (can use up to 50% of a single core)
- **CPU Reservation**: 0.1 cores minimum
- **Memory Limit**: 256MB maximum
- **Memory Reservation**: 64MB minimum

## Available Configuration Options

### 1. CPU Limits

```yaml
cpus: 2.0              # Limit to 2 CPU cores
```

Limits the container to use at most 2 CPU cores. Examples:
- `cpus: 1.0` - Single core
- `cpus: 2.5` - Two and a half cores
- `cpus: 0.5` - Half a core (50% of one core)

### 2. CPU Pinning (Affinity)

To pin containers to specific CPU cores, uncomment and configure:

```yaml
cpuset: "0,1"          # Pin to CPU cores 0 and 1
```

Examples:
- `cpuset: "0,1"` - Use only cores 0 and 1
- `cpuset: "0-3"` - Use cores 0 through 3
- `cpuset: "0,2,4,6"` - Use even-numbered cores

**Note**: Check your NAS CPU topology with `lscpu` to see available cores.

### 3. CPU Shares (Priority)

Control relative CPU priority when system is under load:

```yaml
cpu_shares: 512        # Lower = less priority (default is 1024)
```

Examples:
- `cpu_shares: 2048` - Double priority
- `cpu_shares: 512` - Half priority
- Useful when running multiple containers

### 4. Memory Limits

```yaml
mem_limit: 2g          # Hard limit (container killed if exceeded)
mem_reservation: 512m  # Soft limit (minimum guaranteed)
```

Memory units:
- `b` - bytes
- `k` or `kb` - kilobytes
- `m` or `mb` - megabytes
- `g` or `gb` - gigabytes

## Recommended Settings by NAS Specs

### Low-End NAS (2-4 cores, 4-8GB RAM)
```yaml
backend:
  cpus: 1.0
  mem_limit: 1g
  mem_reservation: 256m

frontend:
  cpus: 0.25
  mem_limit: 128m
  mem_reservation: 32m
```

### Mid-Range NAS (4-8 cores, 8-16GB RAM)
```yaml
backend:
  cpus: 2.0
  mem_limit: 2g
  mem_reservation: 512m

frontend:
  cpus: 0.5
  mem_limit: 256m
  mem_reservation: 64m
```

### High-End NAS (8+ cores, 16GB+ RAM)
```yaml
backend:
  cpus: 4.0
  mem_limit: 4g
  mem_reservation: 1g

frontend:
  cpus: 1.0
  mem_limit: 512m
  mem_reservation: 128m
```

## CPU Pinning Strategy for Unraid

If you want to dedicate specific cores to Glyptotheka while leaving others for Unraid services:

1. **Check available cores**:
   ```bash
   lscpu
   ```

2. **Example: 6-core CPU**
   - Cores 0-1: Unraid system
   - Cores 2-3: Glyptotheka backend
   - Cores 4-5: Other containers

   ```yaml
   backend:
     cpuset: "2,3"
   ```

3. **Avoid core 0** if possible - usually used by system processes

## Additional Optimizations

### 1. I/O Priority

```yaml
blkio_config:
  weight: 500          # Lower = less I/O priority (default 500)
```

### 2. Nice Priority

```yaml
cpu_rt_runtime: 50000  # Microseconds of CPU time in real-time scheduling
```

### 3. PID Limit

Prevent fork bombs:

```yaml
pids_limit: 200        # Maximum number of processes
```

## Monitoring Resource Usage

### Check current usage:
```bash
docker stats glyptotheka-backend glyptotheka-frontend
```

### View detailed resource info:
```bash
docker inspect glyptotheka-backend | grep -A 20 "HostConfig"
```

## Applying Changes

After modifying `docker-compose.yml`:

```bash
# Stop containers
docker-compose down

# Recreate with new limits
docker-compose up -d

# Verify limits
docker inspect glyptotheka-backend | grep -E "CpuShares|Memory|Cpuset"
```

## Troubleshooting

### Container getting killed
- Increase `mem_limit`
- Check logs: `docker logs glyptotheka-backend`
- Look for OOM (Out of Memory) errors

### Slow performance
- Increase `cpus` limit
- Remove CPU pinning if too restrictive
- Increase `mem_reservation`

### High CPU usage during scans
- This is normal - STL preview generation is CPU-intensive
- Consider scheduling scans during off-peak hours
- Reduce `cpus` limit to prevent impacting other services

## Environment Variables for Further Limiting

Add to backend environment:

```yaml
environment:
  - TOKIO_WORKER_THREADS=4     # Limit async runtime threads
  - RAYON_NUM_THREADS=2        # Limit parallel processing threads
```

This works in conjunction with the semaphore limits already implemented in the code.
