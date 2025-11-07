/* 'use client'

import { POST } from '@/app/api/waitlist/route'
import { WaitlistFormData } from '@/types'
import { useState } from 'react'

// ===== STYLES AND CONSTANTS =====
const STYLES = {
  frostContainer: {
    background: 'linear-gradient(115deg, rgba(255, 255, 255, 0.2) 0%, rgba(6, 56, 61, 0.5) 50%, rgba(255, 255, 255, 0.2) 100%)',
    backdropFilter: 'blur(20px)',
  },
  frostBorder: {
    height: "2px", 
    background: "linear-gradient(90deg, #FFF 0%, #2C3A3D 100%)", 
    opacity: 0.2,
    position: 'absolute' as const,
    top: 0,
    right: 0,
    margin: '0 auto',
    inset: 0,
    width: '96%',
  }
}

const CSS_CLASSES = {
  container: "w-full max-w-none mx-auto px-4 sm:px-6 lg:px-8",
  innerContainer: "w-full max-w-4xl mx-auto",
  frostBox: "backdrop-blur-xl rounded-3xl p-6 sm:p-8 lg:p-12",
  formField: "w-full px-4 py-4 bg-white/5 border border-white/10 rounded-xl text-white font-inter focus:outline-none focus:ring-2 focus:ring-orange/50 focus:border-orange/50 transition-all duration-300 backdrop-blur-sm hover:bg-white/10",
  label: "block text-sm font-medium text-white font-inter",
  option: "bg-gray-900 text-white",
  button: "rounded-3xl bg-lime-200 contained-button lg:min-h-[60px] leading-none flex gap-2.5 items-center justify-center font-extrabold font-manrope text-[14px] md:leading-none md:text-xs text-cyan-950 w-full py-4 lg:py-5 px-6 lg:px-8 bg-gradient-to-r from-orange to-orange/90 text-base lg:text-lg transition-all duration-300 relative overflow-hidden group",
  typography: {
    title: "font-bold max-w-[300px] md:max-w-[400px] justify-self-center font-familjen-grotesk italic text-[48px] md:text-[64px] leading-[0.86em] bg-[linear-gradient(121deg,_#FFF_7.14%,_#DEC9F4_82.55%)] text-linear",
    subtitle: "font-familjen-grotesk italic pr-1 text-center text-[24px] md:text-[32px] xl:text-[48px] tracking-[-0.03em] text-linear bg-linear",
    privacy: "text-xs lg:text-sm text-white/50 font-thin mt-6 lg:mt-8 text-center leading-relaxed max-w-2xl mx-auto"
  }
}

const USER_TYPE_OPTIONS = [
  { value: "wallet-developer", label: "Wallet Developer" },
  { value: "defi-developer", label: "DeFi Developer" },
  { value: "product-manager", label: "Product Manager" },
  { value: "founder", label: "Founder/CEO" },
  { value: "investor", label: "Investor" },
  { value: "other", label: "Other" },
]

const INTEREST_OPTIONS = [
  { value: "integration", label: "Integrating yield into my wallet" },
  { value: "strategies", label: "Learning about DeFi strategies" },
  { value: "developer-tools", label: "Developer tools" },
  { value: "partnership", label: "Partnership opportunities" },
  { value: "investment", label: "Potential investment" },
  { value: "learning", label: "Learning about DeFi" },
]

const FEATURES = [
  'Priority access to the platform',
  'Exclusive technical documentation', 
  'Direct support from the team',
  'No commitment, free cancellation'
]

const FrostTopBorder = () => <div style={STYLES.frostBorder} />

const CheckIcon = ({ className }: { className?: string }) => (
  <svg className={className} fill="currentColor" viewBox="0 0 20 20">
    <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
  </svg>
)

const IconWithText = ({ text }: { text: string }) => (
  <div className="flex items-center text-sm lg:text-base text-gray-300 font-inter">
    <div className="w-5 h-5 lg:w-6 lg:h-6 bg-gradient-to-br from-lime to-lime/80 rounded-full flex items-center justify-center mr-3 flex-shrink-0">
      <CheckIcon className="w-4 h-4 lg:w-5 lg:h-5 text-green-700" />
    </div>
    <span>{text}</span>
  </div>
)

const FormContainer = ({ className, children, id }: { className?: string; children: React.ReactNode; id?: string }) => (
  <div className={`${CSS_CLASSES.container} ${className || ''}`} id={id}>
    <div className={CSS_CLASSES.innerContainer}>
      <div className={`${CSS_CLASSES.frostBox} overflow-hidden`} style={STYLES.frostContainer}>
        <FrostTopBorder />
        {children}
      </div>
    </div>
  </div>
)

const FormHeader = ({ title, subtitle }: { title: string; subtitle: string }) => (
  <div className="text-center mb-8 lg:mb-12">
    <h2 className={CSS_CLASSES.typography.title}>
      {title}
    </h2>
    <p className={CSS_CLASSES.typography.subtitle}>
      {subtitle}
    </p>
  </div>
)

const SubmitButton = ({ isSubmitting, children }: { isSubmitting: boolean; children: React.ReactNode }) => (
  <div className="lg:col-span-2">
    <button
      type="submit"
      disabled={isSubmitting}
      className={`
        ${CSS_CLASSES.button}
        ${isSubmitting ? 'opacity-70 cursor-not-allowed' : 'hover:shadow-2xl hover:shadow-orange/25 hover:-translate-y-1 active:translate-y-0'}
      `}
    >
      <span className="relative z-10">
        {children}
      </span>
      <div className="absolute inset-0 bg-gradient-to-r from-white/0 via-white/20 to-white/0 bg-lime-100 transform -translate-x-full group-hover:translate-x-full transition-transform duration-700 ease-out" />
    </button>
  </div>
)

const SuccessMessage = ({ className }: { className?: string }) => (
  <FormContainer className={className}>
    <div className="text-center">
      <div className="w-16 h-16 mx-auto mb-6 rounded-2xl flex items-center justify-center">
        <CheckIcon className="w-8 h-8 text-green-700" />
      </div>
      <h3 className="text-2xl font-bold text-white font-manrope mb-4">
        Thank you for your interest!
      </h3>
      <p className="text-white/75 font-inter leading-relaxed">
        We will contact you soon with exclusive access to DeFindex and all the technical documentation.
      </p>
    </div>
  </FormContainer>
)

interface FormFieldProps {
  id: string
  name: string
  label: string
  type?: string
  required?: boolean
  placeholder?: string
  value: string
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
}

const FormField = ({ id, name, label, type = "text", required, placeholder, value, onChange }: FormFieldProps) => (
  <div className="space-y-2">
    <label htmlFor={id} className={CSS_CLASSES.label}>
      {label} {required && "*"}
    </label>
    <input
      type={type}
      id={id}
      name={name}
      required={required}
      value={value}
      onChange={onChange}
      placeholder={placeholder}
      className={`${CSS_CLASSES.formField} placeholder-gray-400`}
    />
  </div>
)

interface SelectFieldProps {
  id: string
  name: string
  label: string
  required?: boolean
  value: string
  options: { value: string, label: string }[]
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
}

const SelectField = ({ id, name, label, required, value, options, onChange }: SelectFieldProps) => (
  <div className="space-y-2">
    <label htmlFor={id} className={CSS_CLASSES.label}>
      {label} {required && "*"}
    </label>
    <select
      id={id}
      name={name}
      required={required}
      value={value}
      onChange={onChange}
      className={CSS_CLASSES.formField}
    >
      {options.map(option => (
        <option key={option.value} value={option.value} className={CSS_CLASSES.option}>
          {option.label}
        </option>
      ))}
    </select>
  </div>
)

interface CTAFormProps {
  className?: string
  onSubmit?: (data: WaitlistFormData) => void
}

export default function CTAForm({ className = "", onSubmit }: CTAFormProps) {
  const [formData, setFormData] = useState<WaitlistFormData>({
    email: '',
    userType: 'wallet-developer',
    company: '',
    interest: 'integration'
  })

  const [isSubmitting, setIsSubmitting] = useState(false)
  const [isSubmitted, setIsSubmitted] = useState(false)

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormData(prev => ({ ...prev, [name]: value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)

    try {
      if (onSubmit) {
        await onSubmit(formData)
      }
      console.log('Submitting form data:', formData)
      await POST(new Request('/api/waitlist', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData)
      }))
      
      setIsSubmitted(true)
    } catch (error) {
      console.error('Error submitting form:', error)
    } finally {
      setIsSubmitting(false)
    }
  }

  if (isSubmitted) {
    return <SuccessMessage className={className} />
  }

  return (
    <FormContainer className={className} id="cta-form">
      <FormHeader
        title="Join the future of DeFi!"
        subtitle="Be the first to access DeFindex and offer yield-generating accounts to your users."
      />

      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <FormField
            id="email"
            name="email"
            label="Email"
            type="email"
            required
            placeholder="your@email.com"
            value={formData.email}
            onChange={handleInputChange}
          />

          <SelectField
            id="userType"
            name="userType"
            label="What best describes you?"
            required
            value={formData.userType}
            options={USER_TYPE_OPTIONS}
            onChange={handleInputChange}
          />

          <FormField
            id="company"
            name="company"
            label="Company/Project (optional)"
            placeholder="Your company or project name"
            value={formData.company}
            onChange={handleInputChange}
          />

          <SelectField
            id="interest"
            name="interest"
            label="What interests you most about DeFindex?"
            required
            value={formData.interest}
            options={INTEREST_OPTIONS}
            onChange={handleInputChange}
          />
        </div>

        <SubmitButton isSubmitting={isSubmitting}>
          {isSubmitting ? 'Submitting...' : 'Get Exclusive Access'}
        </SubmitButton>
      </form>

      <div className="mt-8 lg:mt-12 pt-8 lg:pt-12 border-t border-white/10">
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 lg:gap-6">
          {FEATURES.map((feature, index) => (
            <IconWithText key={index} text={feature} />
          ))}
        </div>
      </div>

      <p className={CSS_CLASSES.typography.privacy}>
        By signing up, you agree to receive updates about DeFindex. We respect your privacy, and you can unsubscribe at any time.
      </p>
    </FormContainer>
  )
}
 */