# Todo-App 

## DBaaS vs DIY

### DBaaS

Pros:
- Maintenance of the database is handled by service provider
  - Backups
  - Database upgrades
  - Database replication
  - Storage scaling
- Comparatively small amount of set up required
- User-friendly configuration and management interfaces provided by service provider
- Data security and reliability compliance can be achieved immediately

Cons:
- Reduced control over the database software configuration
- System is tied to a specific provider

### DIY

Pros:
- Full control over the database software
  - Versioning
  - Configuration
  - Deployment, scaling and resource usage
- Portable configuration
  - System can be migrated to different cloud providers or into on-prem environments
- No secrets
  - The database system can, if needed, be fully audited
  - The entire software system, including the database, can be reproduced elsewhere and on-demand

Cons:
- Responsibility for maintenance falls on the developer
  - Backup, database upgrade, replication and scaling must be handled directly
  - Achieving compliance with data security and reliability requirements requires extra work
- Larger amount of set up required
- Configuration must be done via tooling provided by the database software
  - Might need to configure everything via CLI or directly via configuration files

### Resolution

For the Todo-App, continuing with a DIY set up makes the most sense, since the DIY setup
has already been created and has been proven to work, so recreating the system with a
DBaaS solution could result in extra work.

Additionally the Todo-App is a simple application with simple requirements. We aren't
expecting to store confidential information or provide multiple nines worth of reliability,
so such compliance guarantees are not valuable. Similarly we aren't interested in backups
or replication, since this project is intended for learning and not as a serious production
project.

Finally, going with a DIY approach serves the purposes of learning Kubernetes better than
using a boxed DBaaS solution which functions outside of the Kubernetes cluster.
