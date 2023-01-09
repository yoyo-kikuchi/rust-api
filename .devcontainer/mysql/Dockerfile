FROM mysql:8.0

ENV LANG=C.UTF-8 \
    TZ=UTC

ENV MYSQL_ROOT_PASSWORD=password \
    MYSQL_ROOT_HOST='%' \
    MYSQL_USER=user \
    MYSQL_PASSWORD=password

ADD ./my.cnf /etc/my.cnf
