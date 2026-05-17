################################################################################
#
# fix-automation
#
################################################################################

FIX_AUTOMATION_VERSION = local
FIX_AUTOMATION_SITE    = $(BR2_EXTERNAL_LINUX_RESCUE_PATH)/..
FIX_AUTOMATION_SITE_METHOD = local

define FIX_AUTOMATION_BUILD_CMDS
	# Binary pre-built by CI — nothing to do here
endef

define FIX_AUTOMATION_INSTALL_TARGET_CMDS
	$(INSTALL) -D -m 0755 \
		$(BR2_EXTERNAL_LINUX_RESCUE_PATH)/../target/x86_64-unknown-linux-musl/release/fix-automation \
		$(TARGET_DIR)/usr/bin/fix-automation

	# Set as PID 1 with environment setup
	echo '#!/bin/sh' > $(TARGET_DIR)/sbin/init
	echo 'export PATH=/sbin:/bin:/usr/sbin:/usr/bin' >> $(TARGET_DIR)/sbin/init
	echo 'export TERM=linux' >> $(TARGET_DIR)/sbin/init
	echo 'echo "Mounting virtual filesystems..."' >> $(TARGET_DIR)/sbin/init
	echo 'mount -t proc proc /proc' >> $(TARGET_DIR)/sbin/init
	echo 'mount -t sysfs sys /sys' >> $(TARGET_DIR)/sbin/init
	echo 'mount -t devtmpfs dev /dev' >> $(TARGET_DIR)/sbin/init
	echo 'echo "Starting Fix-Automation Rescue Environment..."' >> $(TARGET_DIR)/sbin/init
	echo '/usr/bin/fix-automation' >> $(TARGET_DIR)/sbin/init
	echo 'echo "Exiting... rebooting system in 3 seconds."' >> $(TARGET_DIR)/sbin/init
	echo 'sleep 3' >> $(TARGET_DIR)/sbin/init
	echo 'reboot -f' >> $(TARGET_DIR)/sbin/init
	chmod +x $(TARGET_DIR)/sbin/init
endef

$(eval $(generic-package))