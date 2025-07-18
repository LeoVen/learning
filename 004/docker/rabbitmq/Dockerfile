FROM rabbitmq:4.0-management

RUN rabbitmq-plugins enable rabbitmq_management

COPY ./rabbitmq.conf /etc/rabbitmq/rabbitmq.conf
COPY ./definitions.json /etc/rabbitmq/definitions.json
