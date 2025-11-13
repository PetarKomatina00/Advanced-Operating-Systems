# Advanced Operating Systems

This repository contains three independent projects created as part of the **Advanced Operating Systems** course.  
Each project focuses on a different area of system-level development: Linux kernel modification, embedded programming with ZephyrOS, and microservice communication using Docker and RabbitMQ.

## 📌 Projects Overview

### **1. Linux Kernel Modification – Custom System Call**
In this project, I modified the Linux kernel by adding a **custom system call** which prints the current process tree
using a Depth-First Search (DFS) traversal.
The work included:
- Implemented a new system call inside the Linux kernel
- Added a kernel module that interacts with the kernel information
- Created a user space test program that spawns eight processes
- The custom syscall can invoke the test program.  

### **2. ZephyrOS Project – LED Control Based on Internal Temperature**
This project focuses on the embedded real-time operating system **ZephyrOS** and includes both theoretical and practical components.
**Attention!**: For some reason the code does not work on FIT IOT LAB!

#### **Topics covered in the presentation**
1. Growth of IoT devices and related job opportunities  
2. History of ZephyrOS and the size of its GitHub contributor community  
3. Architecture overview with emphasis on **DeviceTree** and its characteristics  
4. Introduction to the **west** tool and its role in Zephyr's build environment  
5. Networking model, including how Zephyr handles sending and receiving data  
6. Design principles related to system **security** in ZephyrOS  

#### **Practical task**
A Zephyr application was implemented on the **nRF52840DK** development board.  
The program:
- reads the board’s internal temperature sensor  
- checks whether the value meets a predefined condition  
- turns on an LED when the threshold is met
  
### **3. Microservices Using Docker and RabbitMQ**
The third project involves building two lightweight microservices designed to communicate using **RabbitMQ**.

Key elements include:
- Two **Rust** based microservices with **Rocket**.
- **Containerized** services, rabbitmq and postgres database using docker and docker-compose.
- Communication between services using **RabbitMQ**
