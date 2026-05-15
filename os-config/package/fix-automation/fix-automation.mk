################################################################################
#
# fix-automation
#
################################################################################

FIX_AUTOMATION_VERSION = local
FIX_AUTOMATION_SITE = $(BR2_EXTERNAL_LINUX_RESCUE_PATH)/..
FIX_AUTOMATION_SITE_METHOD = local

$(eval $(cargo-package))
