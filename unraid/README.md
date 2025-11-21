# Installing Glyptotheka on Unraid

You can install Glyptotheka on Unraid using a standard Docker template. This is the easiest method and does not require any plugins.

## Prerequisites

1.  **Build Image**: The Docker image is automatically built and pushed to GitHub Container Registry (GHCR) when you push to the `main` branch.
    *   Image: `ghcr.io/steffenfriedrich0481/glyptotheka:latest`
    *   Ensure your GitHub repository is **Public** so Unraid can pull the image without authentication.

## Installation Steps

1.  **Copy Template**:
    *   Copy the file `unraid/my-glyptotheka.xml` from this repository to your Unraid USB flash drive at:
        `/boot/config/plugins/dockerMan/templates-user/my-glyptotheka.xml`
    *   You can do this via SMB share (`flash` share), SSH, or the Unraid terminal.

2.  **Add Container**:
    *   Go to the **Docker** tab in Unraid.
    *   Click **Add Container** at the bottom.
    *   In the "Template" dropdown, select **Glyptotheka** (under the "User Templates" section).

3.  **Configure**:
    *   **3D Models Path**: Map this to the share where your STL files are stored (e.g., `/mnt/user/isos/3d-models/`).
    *   **Data Path**: Map this to where you want the database stored (e.g., `/mnt/user/appdata/glyptotheka/data`).
    *   **Cache Path**: Map this to where you want generated previews stored (e.g., `/mnt/user/appdata/glyptotheka/cache`).

4.  **Start**:
    *   Click **Apply**.
    *   Unraid will pull the image and start the container.

The application will be available at `http://<your-unraid-ip>:3000`.

## Alternative: Docker Compose

If you prefer using Docker Compose (e.g., for development or if you want separate containers), you can use the `Docker Compose Manager` plugin with the `docker-compose.yml` file in this directory. However, the single-container template above is recommended for most users.
