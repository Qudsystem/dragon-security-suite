# Database Migrations - Autonomous Mode

## Overview
This directory contains database migrations for the Autonomous Mode feature of Dragon R1.

## Migration Files

### 001_autonomous_mode.sql
Adds complete database schema for autonomous mode:
- Extends `tasks` table with autonomous mode columns
- Creates `constraint_templates` table for constraint definitions
- Creates `autonomous_audit_log` table for complete audit trail
- Creates `constraint_violations` table for violation tracking
- Creates `execution_patterns` table for learning
- Creates `knowledge_updates` table for versioning
- Inserts 5 predefined constraint templates
- Creates views for reporting

## Running Migrations

### Prerequisites
- PostgreSQL database running
- Database credentials configured in `.env` file
- Python 3.8+ with psycopg2 installed

### Apply All Pending Migrations
```bash
cd Cerberus/gateway/migrations
python apply_migrations.py
```

### Check Migration Status
Migrations are tracked in the `schema_migrations` table:
```sql
SELECT * FROM schema_migrations ORDER BY applied_at DESC;
```

### Manual Migration
If needed, you can apply migrations manually:
```bash
psql -h localhost -U dragon_user -d dragon_db -f 001_autonomous_mode.sql
```

## Migration Safety

### Backup Before Migration
Always backup your database before running migrations:
```bash
pg_dump -h localhost -U dragon_user dragon_db > backup_$(date +%Y%m%d_%H%M%S).sql
```

### Rollback
Migrations are tracked and can be rolled back manually if needed. Review the migration file and manually revert changes.

## Predefined Constraint Templates

The migration creates 5 predefined templates:

1. **Conservative**: Reconnaissance only, no exploitation
2. **Balanced**: Low-risk exploitation with strict limits
3. **Aggressive**: Broader exploitation within scope
4. **Web-Only**: Web application testing only
5. **Network-Only**: Network infrastructure testing only

## Database Schema

### New Tables
- `constraint_templates`: Template definitions
- `autonomous_audit_log`: Complete audit trail
- `constraint_violations`: Violation tracking
- `execution_patterns`: Learning patterns
- `knowledge_updates`: Knowledge versioning

### Extended Tables
- `tasks`: Added autonomous mode columns

### Views
- `autonomous_task_summary`: Summary view for reporting

## Indexes
All tables have appropriate indexes for performance:
- Task ID indexes for fast lookups
- Timestamp indexes for time-based queries
- Type/severity indexes for filtering
- Composite indexes for common queries

## Constraints
- Foreign key constraints for referential integrity
- Check constraints for data validation
- Unique constraints for preventing duplicates

## Testing Migrations

After applying migrations, verify:
```sql
-- Check tables exist
SELECT table_name FROM information_schema.tables 
WHERE table_schema = 'public' 
AND table_name LIKE '%autonomous%' OR table_name LIKE '%constraint%';

-- Check predefined templates
SELECT name, description FROM constraint_templates WHERE is_predefined = true;

-- Check indexes
SELECT indexname FROM pg_indexes WHERE tablename IN (
    'constraint_templates', 
    'autonomous_audit_log', 
    'constraint_violations'
);
```

## Troubleshooting

### Migration Fails
1. Check database connection settings
2. Verify user has CREATE TABLE permissions
3. Check for conflicting table names
4. Review error message in `schema_migrations` table

### Rollback Required
1. Backup current database
2. Review migration SQL
3. Manually drop created tables/columns
4. Update `schema_migrations` table

## Next Steps

After migrations are applied:
1. Verify all tables created successfully
2. Check predefined templates loaded
3. Test constraint template CRUD operations
4. Verify audit logging works
5. Test violation tracking
