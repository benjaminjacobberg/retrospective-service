# This script will add the Dev realm and stoodly-service client to keycloak
podman cp ./dev-realm.json container_identity-provider_1:/tmp/dev-realm.json
podman exec -it container_identity-provider_1 /opt/jboss/keycloak/bin/standalone.sh \
-Dkeycloak.profile.feature.upload_scripts=enabled \
-Djboss.socket.binding.port-offset=100 \
-Dkeycloak.migration.action=import \
-Dkeycloak.migration.provider=singleFile \
-Dkeycloak.migration.realmName=dev \
-Dkeycloak.migration.file=/tmp/dev-realm.json