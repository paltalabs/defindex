export interface WaitlistFormData {
  email: string
  userType: 'wallet-developer' | 'defi-developer' | 'product-manager' | 'founder' | 'investor' | 'other'
  company: string
  interest: 'integration' | 'strategies' | 'developer-tools' | 'partnership' | 'investment' | 'learning'
  timestamp?: string
}