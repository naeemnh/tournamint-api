-- Add up migration script here
CREATE TYPE payment_status AS ENUM (
    'pending',
    'processing', 
    'completed',
    'failed',
    'cancelled',
    'refunded',
    'partial_refund'
);

CREATE TYPE payment_method AS ENUM (
    'credit_card',
    'debit_card',
    'paypal',
    'bank_transfer',
    'stripe',
    'other'
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    amount DECIMAL(10,2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    payment_method payment_method NOT NULL,
    status payment_status NOT NULL DEFAULT 'pending',
    transaction_id VARCHAR(255),
    payment_provider VARCHAR(100),
    provider_payment_id VARCHAR(255),
    failure_reason TEXT,
    refunded_amount DECIMAL(10,2) DEFAULT 0.00 CHECK (refunded_amount >= 0),
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Create indexes for better performance
CREATE INDEX idx_payments_user_id ON payments(user_id);
CREATE INDEX idx_payments_tournament_id ON payments(tournament_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_created_at ON payments(created_at);
CREATE INDEX idx_payments_provider_payment_id ON payments(provider_payment_id);
CREATE INDEX idx_payments_transaction_id ON payments(transaction_id);

-- Add constraint to ensure refunded_amount doesn't exceed original amount
ALTER TABLE payments ADD CONSTRAINT chk_refund_amount CHECK (refunded_amount <= amount);