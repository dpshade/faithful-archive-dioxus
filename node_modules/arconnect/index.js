/**
 * Enum for recurring payment frequency
 */
const RecurringPaymentFrequency = {
  QUARTERLY: "Quarterly",
  MONTHLY: "Monthly",
  WEEKLY: "Weekly",
  DAILY: "Daily",
};

/**
 * Enum for subscription status
 */
const SubscriptionStatus = {
  ACTIVE: "Active",
  EXPIRED: "Expired",
  CANCELED: "Canceled",
  AWAITING_PAYMENT: "Awaiting-Payment",
};

module.exports = {
  RecurringPaymentFrequency,
  SubscriptionStatus,
};
