/**
 * Enum for recurring payment frequency
 */
export const RecurringPaymentFrequency = {
  QUARTERLY: "Quarterly",
  MONTHLY: "Monthly",
  WEEKLY: "Weekly",
  DAILY: "Daily",
};

/**
 * Enum for subscription status
 */
export const SubscriptionStatus = {
  ACTIVE: "Active",
  EXPIRED: "Expired",
  CANCELED: "Canceled",
  AWAITING_PAYMENT: "Awaiting-Payment",
};

export default { RecurringPaymentFrequency, SubscriptionStatus };
