# http-download-testing

Code for comparing performance of various Rust libraries against Go HTTP and Curl.

You can run these benchmarks for yourself with:

```bash
docker run -it bennetthardwick/http-download-testing:ubuntu-24.04

export TEST_URL='https://my-file-to-download'

make bench_go
make bench_reqwest
```

## Results

I've been testing this code on Google Cloud Platform. All libraries have similar speed unless running in a Google Kubernetes Engine Autopilot cluster. For some reason Autopilot makes Reqwest go quite slow.

### GCP g1-small, GCS object same region, https

| Lib              | Speed    |
| ---------------- | -------- |
| Go               | 230MiB/s |
| Reqwest          | 210MiB/s |
| Reqwest (Rustls) | 220MiB/s |
| Isahc            | 200MiB/s |
| Ureq             | 250MiB/s |
| Curl             | 260MiB/s |

Run as a docker container.

### GCP GKE Autopilot Pod, n1 machine class, GCS object same region, https

| Lib              | Speed    |
| ---------------- | -------- |
| Go               | 240MiB/s |
| Reqwest          | 104MiB/s |
| Reqwest (Rustls) | 108MiB/s |
| Isahc            | 140MiB/s |
| Ureq             | 230MiB/s |
| Curl             | 220MiB/s |

Kubernetes pod spec:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: http-request-testing
  labels:
    app: http-request-testing
spec:
  nodeSelector:
    cloud.google.com/machine-family: n1
  containers:
  - name: http-request-testing
    image: docker.io/bennetthardwick/http-request-testing:ubuntu-24.04@sha256:9e36ff4d74925e2da1e0258007683b0281bb22f31c7e930dfac3e79b4b8e2f63
    stdin: true 
    tty: true 
    imagePullPolicy: Always
    resources:
      requests:
        cpu: 1
      limits:
        cpu: 1
```
