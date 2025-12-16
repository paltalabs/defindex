'use client';

import { useState } from 'react';
import { gradients } from './styles';

interface FormData {
  name: string;
  email: string;
  company: string;
  telegram: string;
}

interface FormErrors {
  name?: string;
  email?: string;
  company?: string;
}

export default function ContactForm() {
  const [formData, setFormData] = useState<FormData>({
    name: '',
    email: '',
    company: '',
    telegram: '',
  });
  const [errors, setErrors] = useState<FormErrors>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Name is required';
    }

    if (!formData.email.trim()) {
      newErrors.email = 'Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = 'Please enter a valid email';
    }

    if (!formData.company.trim()) {
      newErrors.company = 'Company is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitError(null);

    if (!validateForm()) {
      return;
    }

    setIsSubmitting(true);

    try {
      const response = await fetch('/api/waitlist', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          email: formData.email,
          userType: 'partner',
          interest: `Partner inquiry from ${formData.company}`,
          name: formData.name,
          company: formData.company,
          telegram: formData.telegram || undefined,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to submit form');
      }

      setIsSuccess(true);
      setFormData({ name: '', email: '', company: '', telegram: '' });
    } catch (error) {
      console.error('Error submitting form:', error);
      setSubmitError('Something went wrong. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleInputChange = (field: keyof FormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  if (isSuccess) {
    return (
      <div
        className="rounded-lg p-4 border border-lime-200/30 text-center"
        style={{ background: gradients.cardDark }}
      >
        <div className="w-12 h-12 mx-auto mb-3 rounded-full bg-lime-200/20 flex items-center justify-center">
          <svg
            className="w-6 h-6 text-lime-200"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 13l4 4L19 7"
            />
          </svg>
        </div>
        <h3 className="text-base font-familjen-grotesk font-semibold text-white mb-1">
          Thank You!
        </h3>
        <p className="text-white/75 text-sm">
          We&apos;ve received your inquiry and will be in touch soon.
        </p>
      </div>
    );
  }

  return (
    <div
      className="rounded-lg p-4 border border-cyan-800/50 mt-12"
      style={{ background: gradients.cardDark }}
    >
      {/* Title and Description */}
      <div className="text-center mb-3">
        <h3 className="text-2xl font-familjen-grotesk font-semibold text-white mb-1">
          Request Integration Info
        </h3>
        <p className="text-white/60 text-xs">
          Get in touch with our team to learn more about integrating DeFindex
        </p>
      </div>

      {/* Form Fields - 2 columns on desktop, two rows */}
      <form onSubmit={handleSubmit} className="grid grid-cols-1 md:grid-cols-2 gap-3 items-end">
        
        {/* Left Column */}
        <div className='flex flex-col gap-3 col-span-1'>
         {/* Name */}
          <div>
            <label className="block text-xs font-medium text-white/75 mb-1">
              Name <span className="text-orange-400">*</span>
            </label>
            <input
              type="text"
              value={formData.name}
              onChange={(e) => handleInputChange('name', e.target.value)}
              placeholder="Your name"
              className={`w-full h-9 px-3 text-sm bg-cyan-950 border rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-lime-200 ${
                errors.name ? 'border-orange-400' : 'border-cyan-800'
              }`}
            />
            {errors.name && (
              <p className="mt-0.5 text-[10px] text-orange-400">{errors.name}</p>
            )}
          </div>

          {/* Email */}
          <div>
            <label className="block text-xs font-medium text-white/75 mb-1">
              Email <span className="text-orange-400">*</span>
            </label>
            <input
              type="email"
              value={formData.email}
              onChange={(e) => handleInputChange('email', e.target.value)}
              placeholder="you@company.com"
              className={`w-full h-9 px-3 text-sm bg-cyan-950 border rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-lime-200 ${
                errors.email ? 'border-orange-400' : 'border-cyan-800'
              }`}
            />
            {errors.email && (
              <p className="mt-0.5 text-[10px] text-orange-400">{errors.email}</p>
            )}
          </div>
        </div>
        {/* Right Column */}
        <div className='flex flex-col gap-3 col-span-1'>
          {/* Company */}
          <div>
            <label className="block text-xs font-medium text-white/75 mb-1">
              Company <span className="text-orange-400">*</span>
            </label>
            <input
              type="text"
              value={formData.company}
              onChange={(e) => handleInputChange('company', e.target.value)}
              placeholder="Your company name"
              className={`w-full h-9 px-3 text-sm bg-cyan-950 border rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-lime-200 ${
                errors.company ? 'border-orange-400' : 'border-cyan-800'
              }`}
            />
            {errors.company && (
              <p className="mt-0.5 text-[10px] text-orange-400">{errors.company}</p>
            )}
          </div>

          {/* Telegram */}
          <div>
            <label className="block text-xs font-medium text-white/75 mb-1">
              Telegram <span className="text-white/40 font-normal">(optional)</span>
            </label>
            <input
              type="text"
              value={formData.telegram}
              onChange={(e) => handleInputChange('telegram', e.target.value)}
              placeholder="@username"
              className="w-full h-9 px-3 text-sm bg-cyan-950 border border-cyan-800 rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-lime-200"
            />
          </div>
        </div>

        {/* Submit Error */}
        {submitError && (
          <div className="md:col-span-4 p-2 rounded-lg bg-orange-400/10 border border-orange-400/30">
            <p className="text-xs text-orange-400">{submitError}</p>
          </div>
        )}

        {/* Submit Button - Full Width */}
        <div className="md:col-span-4">
          <button
            type="submit"
            disabled={isSubmitting}
            className="w-full h-9 bg-lime-200 text-cyan-900 text-sm font-semibold rounded-lg hover:bg-lime-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSubmitting ? (
              <span className="flex items-center justify-center gap-2">
                <svg
                  className="animate-spin h-4 w-4"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                Sending...
              </span>
            ) : (
              'Request Integration Info'
            )}
          </button>
        </div>
      </form>
    </div>
  );
}
