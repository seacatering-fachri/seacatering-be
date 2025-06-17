
-- ================================
-- USERS TABLE
-- ================================
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE
);

-- ================================
-- MEAL PLANS TABLE
-- ================================
CREATE TABLE meal_plans (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    price_per_meal INTEGER NOT NULL
);

-- ================================
-- MEAL TYPES TABLE
-- ================================
CREATE TABLE meal_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);

-- ================================
-- DELIVERY DAYS TABLE
-- ================================
CREATE TABLE delivery_days (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);

-- ================================
-- SUBSCRIPTIONS TABLE
-- ================================
CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    meal_plan_id INTEGER NOT NULL REFERENCES meal_plans(id),
    allergies TEXT,
    total_price INTEGER NOT NULL,
    status VARCHAR(20) DEFAULT 'Active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ================================
-- SUBSCRIPTION MEAL TYPES (Join Table)
-- ================================
CREATE TABLE subscription_meal_types (
    subscription_id INTEGER REFERENCES subscriptions(id) ON DELETE CASCADE,
    meal_type_id INTEGER REFERENCES meal_types(id),
    PRIMARY KEY (subscription_id, meal_type_id)
);

-- ================================
-- SUBSCRIPTION DELIVERY DAYS (Join Table)
-- ================================
CREATE TABLE subscription_delivery_days (
    subscription_id INTEGER REFERENCES subscriptions(id) ON DELETE CASCADE,
    delivery_day_id INTEGER REFERENCES delivery_days(id),
    PRIMARY KEY (subscription_id, delivery_day_id)
);

-- ================================
-- TESTIMONIALS TABLE
-- ================================
CREATE TABLE testimonials (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ================================
-- INSERT DEFAULT MEAL PLANS
-- ================================
INSERT INTO meal_plans (name, price_per_meal) VALUES
('Diet Plan', 30000),
('Protein Plan', 40000),
('Royal Plan', 60000);

-- ================================
-- INSERT DEFAULT MEAL TYPES
-- ================================
INSERT INTO meal_types (name) VALUES
('Breakfast'), ('Lunch'), ('Dinner');

-- ================================
-- INSERT DEFAULT DELIVERY DAYS
-- ================================
INSERT INTO delivery_days (name) VALUES
('Monday'), ('Tuesday'), ('Wednesday'), ('Thursday'),
('Friday'), ('Saturday'), ('Sunday');
