# Kubernetes

## Challanges in Containerization

- Failures such as container crashes
- Scheduling containers to specific machines depending on configuration
- Upgrading and rolling back the applications
- Scaling up and down containers across the set of machines.

## Introduction 

- Kubernetes is an open-source and portable platform
- Container orchestration  tool, which automates scaling of workloads
- Groups containers into logical units.
- Has a master node, and multiple worker nodes
- Each worker node can manage mulitple containers
- Can self-heal containized applications
- can auto-scale

## OKE

- Full form : Container Engine for Kubernetes
- OKE is ISO compliant

### Components

- Cluster Control Pane : 
    - Oracle managed
    - manages kube-controller-manager, cloud-controller-manager, kube-APIServer, and kube-scheduler
- Cluster Data Pane : 
    - User managed
    - Nodes

### Types of Clusters

- Basic Clusters : 
    - Supports all the core functionalities of kubernetes and container engine for kubernetes OKE
    - Comes with Service level Objective (SLO) but not a financially backed Service level Agreement (SLA)
    - Oracle is responsible for anything but not financially
- Enhanced Clusters :
    - Simplified node management using virtual nodes
    - Deploy and configure cluster add-ons in a more granular way
    - Provision more worker nodes in a cluster
    - Strengthen security by using workload identity
    - Rely on the SLA i.e. financially backed.
    - Default option, when creating a cluster using console
    - Default option is basic, when creating a cluster using an API or CLI.
    - Can't downgrade, can upgrade.

## Serverless Kubernetes

### Managed Nodes 

- Managed nodes run on compute instances (bare metal or virtual machines) in your tenancy.
- Partly managed by the customer
- Two most important components are kubelet, and the container runtime.
- Workloads can be targeted to specific nodes based on their requirements.

### Virtual Nodes

- Fully managed and highly available nodes.
- Acts like real nodes of kubernetes
- Built using open-source CNCF Virtual Kubelet Project.
- Configured by customers and are located within a single availability and fault domain within OCI.
- Two main components, pod management and container instance management.
- Delegates all responsibility of managing the lifecycle of pods to Virtual Kubelet.
- Supports 1000 pods.

## OKE Clusters

### Prerequisites

- Access to the OCI tenancy
- Sufficient Quotas on resources (service limits)
    - Compute Instance quota : At least one compute instance (node) must be available at the tenancy
    - Block volume quota : Persistent volume claims must request a minimum of 50 GB
    - Load Balencer Quota : Sufficient load balencer quota must be available in the region
    - VCN and Subnet Support
- Ready-to-deploy comparment : 
    - Best practice is to create a seperate compartment for each team.
- Configuring network resources : 
    - Includes VCNs, subnets, internet gateway, route table, security lists
    - Container engine for Kubernetes offers options to automatically create and configure new network resources.
- Policies :   
    - To create/manage clusters, you must belong to the tenancy's Admin group or group with adequate permissions
- Tools : Kubectl and KubeConfig : 
    - Kubectl installation is included in Cloud Shell
    - You must set up your own copy of cluster's Kubeconfig configuration file

### Policy Configuration for OKE Clusters

- When a tenancy is created, an admin group is automatically created.
- IAM Policies required for clusters : 
    - Allow group <group_name> to use subnets in <location>
    - Manage instances
    - Use network-security-groups
    - read virtual network family
    - Use vnics
    - Inspect compartments
    - Use private-ips
    - manage public-ips
    - Manage vncs
    - Manage internet-gateways, nat-gateways
    - Manage route-tables, security lists
- Optional Policies : 
    - Cloud Shell policy : Allow group <group-name> to use cloud-shell in <location>
    - Vault : to read vault and keys
    - Cluster : Inspect clusters, use cluster-node-pools, read cluster-work-requests
    - Service gateways : manage service-gateways
    - Capacity reservation : use compute-capacity-reservations

### Creating Kubernetes clusters using Console Workflows

- Ways to create OKE Cluster :
    - Quick Create Workflow using default settings
    - Custom create workflow using custom settings

- Quick create workflow
    - Utilize default settings to swiftly create a cluster with new network resources
    - Accepting default values allows creating a cluster with a few clicks.
    - Automatic network resources creation including regional subnets for kubernetes API, worker nodes and load balancers
    - Membership in a policy-granted group needed for new network resource creation.
- Custom Create workflow : 
    - Offers maximum cluster control with custom settings.
    - Tailor new cluster properties, including encryption
    - Precise network resource selection using existing public or private subnets for kubernetes API, worker nodes and load balancers
    - Define node pools immediately or delay, even create a pool-free cluster.
    - Create a node-pool-free cluster facilitates Calico installation without pod recreation. (Best practice)
- Can be created using CLI and API too. 

### Accessing clusters using Kubectl

- Either using cloud shell or your local installation
- There should be a .config file, showing the tool which cluster to look after. Under .kube directory
- Config file :
    - Single kubeconfig file can manage details for multiple clusters as different contexts
    - File includes the OCI CLI command for generating and inserting authentication tokens when using kubectl
    - Authentication tokens are generated by OCI CLI in the kubeconfig are short-lived, cluster-specific and user-specific
    - Kubeconfig files cannot be shared among users.

### Accessing clusters using Cloudshell

- API endpoint has a public IP address
- Access your cluster dialog in the OKE console provides the setup command.
- If a kubeconfig file already exists, new cluster details are added as a new-context to the existing kubeconfig file, and the current context is updated.
- To use a custom kubeconfig file or location, set the KUBECONFIG env variable.
- Verify that kubectl can connect to the cluster using `kubectl get nodes` command.
- Access cluster from local terminal when cluster API endpoint has not public IP, with VCN network peering. 
    - Generate an API signing key pair
    - Upload the public key of the API signing key pair
    - Install and configure the OCI CLI
    - Setup the kubeconfig file :  
        - Kubectl version must be within one minor version (older or newer) of the Kubernetes version running on the control plane nodes.
        - Create a directory for kubeconfig, typically named $HOME/.kube by default
    - Run `kubectl version` command to verify installation

## OKE Self Managed Nodes

- Not pooled in node pools as default, BYON
- Has specialized configurations like GPU shapes and high-frequency processor cores.
- Complete control
- Provides high-bandwidth, low-latency, RDMA connectivity.

### Prerequisites

- Confirm that the OKE Cluster is configured properly
- Select an appropriate image for the compute instance hosting the self-managed node
- Cluster requirements : 
    - Should be an enhanced cluster.
    - Flannel CNI plugin for pod networking should be used, not VCN-Native Pod Networking CNI Plugin.
    - Control Pane nodes should be running Kubernetes version 1.25 or later.
    - Ensure compatibility between Kubernetes versions in control plane nodes and worker nodes.
- Image requirements : 
    - Select either Oracle Linux 7 (OL7) or OL8 image with release date of Mar 28,2023 or later.
    - Get the image OCID from OKE Worker Node Oracle Linux 7.x Images or OKE Worker Node Oracle Linux 8.x Images
    - Kubernetes version compatibility

## OCI Service Operator for Kubernetes

- Open source kubernetes add-on, which facilitates the management and connection of OCI resources.
- Enables you to create, configure and interact with OCI resources natively within kubernetes clusters.
- Eliminates the need for OCI console, CLI or other tools
- Interact with the services by using kubectl to call the Operator Framework APIs
- Operators have :
    - Operator SDKs : Uses kubernetes controller runtime library for high-level APIs, abstractions, scaffolding and code generation.
    - Operator Lifecycle Manager (OLM) : Extends kubernetes to declaratively install, manage and upgrade operators within clusters.
