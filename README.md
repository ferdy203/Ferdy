# message-broker

## Project Components 
- [ ] Message Broker
- [ ] API Client - NodeJs/Java
- [ ] Examples

## Features
### Topic 
- [ ] Same message can be sent to different topic using Wildcard Routing, depending on message configuration just like [RabbitMQ Topic](https://www.rabbitmq.com/tutorials/tutorial-five-python.html)
- [ ] Dead later topic
- [ ] Messages expiry
- [ ] Option for message durability on topic level and message level
- [ ] Same consumer can bound himself to multiple topics, in that case topic name must be delived to consumer too
- [ ] Option for multiple consumer to be bounded to the same binding key. In this case, all consumers can choose to either load balance messages between them or they all want to receive a copy of message
- [ ] Message buffering - same messages in temporary storage till any consumer is up to receive the message or expiry time(Keep a default expiry time). 
- [ ] Consumer Acknowledgments
- [ ] Producer Acknowledgments
- [ ] Duplicate Message handling(bloom filter)
### Queue
- [ ] Message buffering
- [ ] Message Persistence
- [ ] Durable Queues
- [ ] Consumer Acknowledgments
- [ ] Producer Acknowledgments
- [ ] Message Time-to-Live (TTL):
- [ ] Message Dead-Lettering
- [ ] Message Priority
- [ ] Message Queue Length Limit:
- [ ] Load Balancing: If multiple consumers are connected to the same queue, messages are distributed among consumers in a round-robin fashion, achieving load balancing
- [ ] Queue Auto-Deletion: Queues can be configured to be automatically deleted after a period of inactivity. This feature is useful for temporary queues that are used for short-lived tasks.
- [ ] Duplicate Message handling(bloom filter)

## Prerequisite to complete before starting the project
- [ ] Project Reactor + Reactive Streams
- [ ] AMQP Understanding
- [ ] Apache Avro
- [x] Apache ZooKeeper
- [x] Plain TCP Communication in Java'
     - [ ] TCP with NIO
     - [ ] TCP with reactor
- [ ] Encoding and Decoding 
- [ ] Schema Evolution

## Non Functional Requirement 
- [ ] It should be scalable as and when required
- [ ] Foult tolerant
- [ ] It should have retry mechanism
- [ ] Everything should be configurable at run time, no need to start the message broker, consumer or producer

