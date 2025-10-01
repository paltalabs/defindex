export interface WaitlistFormData {
  email: string
  userType: 'wallet-developer' | 'defi-developer' | 'product-manager' | 'founder' | 'investor' | 'other'
  company: string
  interest: 'integration' | 'strategies' | 'developer-tools' | 'partnership' | 'investment' | 'learning'
  timestamp?: string
}

// Component types
export interface Strategy {
  id: number;
  title?: string;
  icon?: string;
  description: string;
}

export interface SocialLink {
  icon: React.ElementType;
  url: string;
  label?: string;
}

export interface SecurityFeature {
  id: number;
  icon: string;
  title: string;
  description: string;
}

export interface FAQItem {
  title: string;
  description: string;
  isOpen?: boolean;
}